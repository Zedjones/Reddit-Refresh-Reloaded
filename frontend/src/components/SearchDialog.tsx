import { useEffect } from 'react';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogTitle from '@mui/material/DialogTitle';
import * as yup from 'yup';
import { useFormik } from 'formik';
import { useAddSearchMutation } from '../types';
import { validateDuration } from './Login';
import { values } from 'lodash';
import SnackbarUtils from './SnackbarUtils';

interface SearchDialogProps {
  subreddit?: string;
  open: boolean;
  setOpen: (newVal: boolean) => void;
};

interface SearchDialogFormValues {
  subreddit: string;
  searchTerm: string;
  refreshTime?: string;
}

export default function SearchDialog(props: SearchDialogProps) {
  const { open, setOpen } = props;
  const [addSearchResult, addSearch] = useAddSearchMutation();
  const initialValues: SearchDialogFormValues = { subreddit: props.subreddit ?? '', searchTerm: '', refreshTime: '' };
  const formik = useFormik({
    initialValues,
    validationSchema: yup.object({
      subreddit: yup.string().required('A subreddit to search is required'),
      searchTerm: yup.string().required('A search term is required'),
      refreshTime: yup.string().optional().test('validate-duration', 'Must be in Rust/Go duration format, e.g. 1d15h5m4s, and be at least 5 seconds', value => {
        return value ? validateDuration(value) : true;
      })
    }),
    onSubmit: values => {
      addSearch({
        ...values,
        refreshTime: values.refreshTime === '' ? undefined : values.refreshTime
      });
      setOpen(false);
    }
  });

  const handleClose = () => {
    setOpen(false);
  };

  // Don't have Formik as a depdendency to prevent an infinite loop
  useEffect(() => {
    formik.setFieldValue('subreddit', props.subreddit ?? '');
  }, [props.subreddit]); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    if (addSearchResult.data) {
      SnackbarUtils.success('Search was successfully created.');
    }
  }, [addSearchResult])

  return (
    <div>
      <Dialog open={open} onClose={handleClose} TransitionProps={{
        onExited: () => formik.resetForm()
      }}>
        <form onSubmit={formik.handleSubmit}>
          <DialogTitle>Subscribe</DialogTitle>
          <DialogContent>
            <DialogContentText>
              To subscribe to this website, please enter your email address here. We
              will send updates occasionally.
            </DialogContentText>
            <TextField
              autoFocus={props.subreddit === undefined}
              value={formik.values.subreddit}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.subreddit && Boolean(formik.errors.subreddit)}
              helperText={formik.touched.subreddit && formik.errors.subreddit}
              margin="dense"
              id="subreddit"
              label="Subreddit"
              type="text"
              fullWidth
              variant="standard"
            />
            <TextField
              autoFocus={props.subreddit !== undefined}
              value={formik.values.searchTerm}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.searchTerm && Boolean(formik.errors.searchTerm)}
              helperText={formik.touched.searchTerm && formik.errors.searchTerm}
              margin="dense"
              id="searchTerm"
              label="Search Term"
              type="search"
              fullWidth
              variant="standard"
            />
            <TextField
              value={formik.values.refreshTime}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              error={formik.touched.refreshTime && Boolean(formik.errors.refreshTime)}
              helperText={formik.touched.refreshTime && formik.errors.refreshTime}
              margin="dense"
              id="refreshTime"
              label="Refresh Time"
              type="text"
              fullWidth
              variant="standard"
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={handleClose}>Cancel</Button>
            <Button type="submit">Submit</Button>
          </DialogActions>
        </form>
      </Dialog>
    </div>
  );
}