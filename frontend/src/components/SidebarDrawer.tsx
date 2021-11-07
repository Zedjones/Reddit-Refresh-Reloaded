import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListSubheader from '@material-ui/core/ListSubheader';
import { ListItemText } from '@material-ui/core';

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
        {['File', 'Pushbullet', 'Gotify'].map(text => (
          <ListItem button key={text}>
            <ListItemText primary={text} />
          </ListItem>
        ))}
      </List>
    </Drawer>
  )
}