import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useGetSearchResultsQuery } from "../types";
import SidebarDrawer from "./SidebarDrawer";

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
    <>
      <SidebarDrawer />
      {results.data?.getSearch.results.map(result => <div />)}
    </>
  )
}