import React, { useState } from 'react';
import SearchDialog from '../components/SearchDialog';
import Button from '@material-ui/core/Button';

export default {
  title: 'Search Dialog',
  component: SearchDialog,
}

export const ToStorybook = () => {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button onClick={() => setOpen(true)}>
        Click me
      </Button>
      <SearchDialog subreddit="mechmarket" open={open} setOpen={setOpen} />
    </>
  )
}

ToStorybook.story = {
  name: 'Search Dialog'
}