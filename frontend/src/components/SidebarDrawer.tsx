import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListSubheader from '@material-ui/core/ListSubheader';
import { ListItemText } from '@material-ui/core';
import { IconButton } from '@material-ui/core';
import { useGetUserSettingsQuery, useGetUserSearchesQuery, useDeleteSearchMutation } from '../types';
import { Button, ListItemButton, ListItemIcon } from '@mui/material';
import SearchIcon from '@mui/icons-material/Search';
import RedditIcon from '@mui/icons-material/Reddit';
import AddIcon from '@mui/icons-material/AddCircleOutline';
import DeleteIcon from '@mui/icons-material/Delete';
import { useEffect, useMemo, useState } from 'react';
import * as _ from 'lodash';
import SearchDialog from './SearchDialog';
import SnackbarUtils from './SnackbarUtils';

const NotifierItems = () => {
  const [result, refetch] = useGetUserSettingsQuery();

  const { data, fetching } = result;

  if (fetching) return <> </>;

  return (
    <div>
      {(data?.getUserInfo.settings.map(setting =>
        <ListItem button key={`setting-${setting.id}`}>
          <ListItemText primary={setting.name} />
        </ListItem>)) ?? []}
    </div>
  );
}

export default function SidebarDrawer() {
  const [result, refetch] = useGetUserSearchesQuery();
  const [deleteResult, deleteSearch] = useDeleteSearchMutation();
  const [searchDialogSub, setSearchDialogSub] = useState<string | undefined>();
  const [dialogOpen, setDialogOpen] = useState(false);

  const { data, fetching } = result;

  // Function to create an object mapping subreddit name to search
  const sortBySub = useMemo(() => {
    const searchArray = data?.getUserInfo.searches ?? [];
    const grouped = _.groupBy(searchArray, 'subreddit');
    const mapped = _.mapValues(grouped, search => search.map(key => _.omit(key, 'subreddit')));
    return mapped;
  }, [data]);

  const setOpen = (newValue: boolean) => {
    if (!newValue) {
      setSearchDialogSub(undefined);
    }
    setDialogOpen(newValue);
  };

  useEffect(() => {
    if (deleteResult.data) {
      SnackbarUtils.success('Successfully deleted search.');
    }
  }, [deleteResult]);

  return (
    <Drawer open>
      <SearchDialog subreddit={searchDialogSub} open={dialogOpen} setOpen={setOpen} />
      <List>
        <ListSubheader>
          Searches
          <IconButton sx={{ alignContent: 'stretch' }} onClick={() => {
            setOpen(true);
          }}>
            <AddIcon />
          </IconButton>
        </ListSubheader>
        <List component="div" key='Subreddits' disablePadding>
          {Object.entries(sortBySub).map(([sub, search]) => (
            <>
              <ListItem key={sub} sx={{ pl: 4 }}>
                <ListItemIcon>
                  <RedditIcon />
                </ListItemIcon>
                <ListItemText primary={sub} />
                <IconButton sx={{ alignContent: 'stretch' }} onClick={() => {
                  setSearchDialogSub(sub);
                  setOpen(true);
                }}>
                  <AddIcon />
                </IconButton>
              </ListItem>
              <List component="div" key={`${sub}-list`} disablePadding>
                {search.map(search => (
                  <ListItem key={search.searchTerm} sx={{ pl: 8 }} secondaryAction={
                    <IconButton edge="end" onClick={() => {
                      deleteSearch({
                        id: search.id
                      });
                    }}>
                      <DeleteIcon />
                    </IconButton>
                  }>
                    <ListItemButton>
                      <ListItemIcon>
                        <SearchIcon />
                      </ListItemIcon>
                      <ListItemText primary={search.searchTerm} />
                    </ListItemButton>
                  </ListItem>))}
              </List>
            </>
          ))}
        </List>
        <ListSubheader>
          Notifier Preferences
        </ListSubheader>
        <NotifierItems />
      </List >
    </Drawer >
  )
}