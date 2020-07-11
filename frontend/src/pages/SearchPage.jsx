import React from 'react';
import Fab from '@material-ui/core/Fab';
import { makeStyles } from '@material-ui/core/styles';
import AddIcon from '@material-ui/icons/Add';

const useStyles = makeStyles(theme => ({
  fab: {
    position: 'absolute',
    bottom: theme.spacing(2),
    right: theme.spacing(2),
  }
}))

export default function SearchPage() {
  const classes = useStyles();
  return (
    <Fab className={classes.fab}>
      <AddIcon />
    </Fab>
  )
}