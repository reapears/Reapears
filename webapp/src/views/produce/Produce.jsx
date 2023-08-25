import React, { useState } from "react";

import {
  makeStyles,
  shorthands,
  Skeleton,
  SkeletonItem,
} from "@fluentui/react-components";

import { useQuery } from "react-query";
import { getHarvests } from "../../endpoint";
import { HarvestCard } from "./HarvestCard";

const useStyles = makeStyles({
  main: {
    display: "flex",
    flexWrap: "wrap",
    columnGap: "30px",
    rowGap: "36px",
  },

  title: {
    ...shorthands.margin(0, 0, "12px"),
  },

  description: {
    ...shorthands.margin(0, 0, "12px"),
  },

  card: {
    width: "250px",
    maxWidth: "100%",
    // height: "fit-content",
    // height: "360px",
  },

  text: {
    ...shorthands.margin(0),
  },
});

export function Produce(props) {
  const styles = useStyles();
  const [query, setQuery] = useState({
    cultivar: [],
    region: [],
    offset: [],
  });

  const { isLoading, data: response } = useQuery("produce", () =>
    getHarvests(query)
  );

  if (isLoading) {
    return (
      <Skeleton {...props}>
        <SkeletonItem />
      </Skeleton>
    );
  }

  const { harvests, offset } = response.data;

  return (
    <main>
      <h2>Produce page</h2>

      <div className={styles.main}>
        {harvests.map((harvest) => {
          return <HarvestCard key={harvest.id} harvest={harvest} />;
        })}
      </div>
    </main>
  );
}
