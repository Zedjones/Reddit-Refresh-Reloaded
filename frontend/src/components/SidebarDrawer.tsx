import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListSubheader from '@material-ui/core/ListSubheader';
import { ListItemText } from '@material-ui/core';
import { useGetUserSettingsQuery } from '../types';

const NotifierItems = () => {
  const [result, refetch] = useGetUserSettingsQuery();

  const { data, fetching, error } = result;

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
  return (
    <Drawer open>
      <List>
        {['Searches', 'Results'].map(text => (
          <ListItem button key={text}>
            <ListItemText primary={text} />
          </ListItem>
        ))}
        <ListSubheader>
          Notifier Preferences
        </ListSubheader>
        <NotifierItems />
      </List>
    </Drawer>
  )
}