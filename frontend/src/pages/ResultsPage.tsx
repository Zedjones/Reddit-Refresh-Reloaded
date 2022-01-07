import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { GetSearchResultsQuery, useGetSearchResultsQuery } from "../types";
import SidebarDrawer from "../components/SidebarDrawer";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";
import { Box, Button, CardMedia, Grid } from "@mui/material";
import { CardActions } from "@material-ui/core";
import RRAppBar, { AppBarWrapper } from '../components/AppBar';

type ResultsQueryItem = GetSearchResultsQuery['getSearch']['results'][0];

// https://stackoverflow.com/questions/5717093/check-if-a-javascript-string-is-a-url
function isValidHttpUrl(potentialURL: string | null) {
  try {
    if (!potentialURL) {
      return false;
    }
    const url = new URL(potentialURL);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch (_) {
    return false;
  }
};

const ResultCard = (search: ResultsQueryItem) => {
  // Have to multiply timestamp by 1000 for some reason
  const insertedDate = new Date(search.inserted * 1000);
  return (
    <Box mt={3}>
      <Card sx={{ minWidth: 275 }}>
        {search.thumbnail && isValidHttpUrl(search.thumbnail) &&
          <CardMedia
            component="img"
            image={search.thumbnail}
            height="194"
          />}
        <CardContent>
          <Typography gutterBottom variant="h5" component="div">
            {search.title}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {insertedDate.toString()}
          </Typography>
        </CardContent>
        <CardActions>
          <Button target="_blank" href={search.permalink} size="small">Open</Button>
        </CardActions>
      </Card>
    </Box>
  )
}

const drawerWidth = 350;

export function ResultsPage() {
  const { searchId } = useParams();
  const [parsedId, setParsedId] = useState(0);
  const navigate = useNavigate();

  // If the searchId isn't a number, just navigate away for now
  // TODO: Maybe show an error page or something?
  useEffect(() => {
    const parsed = parseInt(searchId!);
    if (isNaN(parsed)) {
      navigate("/");
    }
    setParsedId(parsed);
  }, [searchId, navigate]);

  const [results, refetch] = useGetSearchResultsQuery({
    variables: {
      id: parsedId
    },
    requestPolicy: "cache-and-network"
  });

  return (
    <>
      <RRAppBar />
      <Box sx={{ display: 'flex' }}>
        <Box
          component="nav"
          sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
          aria-label="mailbox folders"
        >
          <SidebarDrawer drawerWidth={drawerWidth} />
        </Box>
        <Box
          component="main"
          sx={{ flexGrow: 1, p: 3, width: { sm: `calc(100% - ${drawerWidth}px)` } }}
        >
          {results.data?.getSearch.results.map(ResultCard)}
        </Box>
      </Box>
    </>
  )
}

const WrappedResults = () => <AppBarWrapper>
  <ResultsPage />
</AppBarWrapper>;

export default ResultsPage;