import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { GetSearchResultsQuery, useGetSearchResultsQuery } from "../types";
import SidebarDrawer from "../components/SidebarDrawer";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";
import { Box, CardMedia } from "@mui/material";

type ResultsQueryItem = GetSearchResultsQuery['getSearch']['results'][0]

const ResultCard = (search: ResultsQueryItem) => {
  // Have to multiply timestamp by 1000 for some reason
  const insertedDate = new Date(search.inserted * 1000);
  return (
    <Card sx={{ minWidth: 275 }}>
      <CardMedia>

      </CardMedia>
      <CardContent>
        <Typography gutterBottom variant="h5" component="div">
          {search.title}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          {insertedDate.toString()}
        </Typography>
      </CardContent>
    </Card>
  )
}

const drawerWidth = 350;

export default function ResultsPage() {
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
    }
  });

  useEffect(() => console.log(results), [results]);

  return (
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
  )
}