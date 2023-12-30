import React, { useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import {
  Skeleton,
  SkeletonItem,
  Tab,
  TabList,
  Image,
  Text,
  makeStyles,
  Divider,
} from "@fluentui/react-components";

import { Footer, Header } from "../../components";
import { useInfiniteQuery } from "react-query";
import { getHarvests } from "../../endpoint";
import { HarvestCard } from "./HarvestCard";
import { cultivarIconNames, cultivarIcons } from "./CultivarIcons";
import { selectCultivarFilter } from "../../store";

export function Produce(props) {
  const harvestFilter = useSelector((state) => state.cultivarFilter);

  // Api requests
  const [query, setQuery] = useState({
    cultivar: [],
    region: [],
  });

  const [cultivarFilter, setCultivarFilter] = useState("");

  const fetchHarvests = ({ pageParam = undefined }) => {
    const params = pageParam ? { ...query, offset: [pageParam] } : query;
    return getHarvests(params);
  };

  const { data, error, fetchNextPage, hasNextPage, status } = useInfiniteQuery(
    "produce",
    fetchHarvests,
    {
      getNextPageParam: (lastPage, pages) => {
        if (lastPage.data.harvests) {
          return lastPage.data.offset;
        }
      },
    }
  );

  // Auto load more harvests.
  window.addEventListener("scroll", () => {
    const { scrollHeight, scrollTop, clientHeight } = document.documentElement;
    if (scrollTop + clientHeight >= scrollHeight && hasNextPage) {
      fetchNextPage();
    }
  });

  // Renders
  if (status === "loading") {
    return (
      <>
        <div className="produce-header sticky ">
          <Header />
          <Divider />
          <ProduceFilters
            cultivarFilter={cultivarFilter}
            setCultivarFilter={setCultivarFilter}
          />
        </div>
        {/* <Skeleton {...props}>
          <SkeletonItem />
        </Skeleton> */}
      </>
    );
  } else if (status === "error") {
    return (
      <>
        <div className="produce-header sticky ">
          <Header />
          <Divider />
          <ProduceFilters
            cultivarFilter={cultivarFilter}
            setCultivarFilter={setCultivarFilter}
          />
        </div>
        {/* <p>{cultivarFilter}</p> */}
        <p>Error: {error.message}</p>
      </>
    );
  }

  return (
    <>
      <div className="produce-header sticky ">
        <Header />
        <Divider />
        <ProduceFilters
          cultivarFilter={cultivarFilter}
          setCultivarFilter={setCultivarFilter}
        />
      </div>

      <div className="container">
        <div className="produce-container">
          {data.pages.map((response, i) => (
            <React.Fragment key={i}>
              {response.data.harvests
                .filter((harvest) => {
                  return (
                    !harvestFilter.filter ||
                    harvestFilter.filter === "Harvests" ||
                    harvest.name === harvestFilter.filter
                  );
                })
                .map((harvest) => {
                  return <HarvestCard key={harvest.id} harvest={harvest} />;
                })}
            </React.Fragment>
          ))}
        </div>
      </div>
      <Footer />
    </>
  );
}

function ProduceFilters(props) {
  const harvestFilter = useSelector((state) => state.cultivarFilter);
  const dispatch = useDispatch();
  const onTabSelect = (event, data) => {
    dispatch(selectCultivarFilter(data.value));
  };

  return (
    <div className="cultivar-filter">
      <TabList
        selectedValue={harvestFilter.filter}
        onTabSelect={onTabSelect}
size="large"
        {...props}
      >
        {cultivarIconNames.map((name) => {
          return (
            <CultivarFilter key={name} name={name} icon={cultivarIcons[name]} />
          );
        })}
      </TabList>
    </div>
  );
}

const useStyles = makeStyles({
  cultivarFilterTabIcon: {
    width: "32px",
    height: "32px",
  },
});

function CultivarFilter({ name, icon }) {
  const styles = useStyles();

  return (
    <Tab value={name}>
      <div className="cultivar-filter--tab">
        <div className="cultivar-filter--tab-icon">
          <Image
            className={styles.cultivarFilterTabIcon}
            src={icon}
            alt={`${name} icon`}
            fit="cover"
          />
        </div>

        <div className="cultivar-filter--tab-title">
          <Text>{name}</Text>
        </div>
      </div>
    </Tab>
  );
}
