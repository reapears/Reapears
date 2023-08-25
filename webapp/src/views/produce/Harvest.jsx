import React from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useQuery } from "react-query";

import {
  Persona,
  Text,
  Skeleton,
  SkeletonItem,
  Subtitle1,
} from "@fluentui/react-components";
import { Price } from "../../components";
import { getHarvest, userPhotoResolver } from "../../endpoint";

export function Harvest(props) {
  const params = useParams();
  const navigate = useNavigate();
  const harvestId = params["harvestId"];

  const {
    isLoading,
    isError,
    data: response,
  } = useQuery(["harvest", harvestId], () => getHarvest(harvestId));

  if (isLoading) {
    return (
      <Skeleton {...props}>
        <SkeletonItem />
      </Skeleton>
    );
  }

  const harvest = response.data;
  const { farmOwner } = harvest;

  function toUserProfile() {
    const userId = farmOwner.id;
    return navigate(`/user/${userId}/profile`);
  }

  return (
    <div>
      <div>
        <Subtitle1>Harvest Info</Subtitle1>
        <ul>
          <li>
            <Text>{harvest.name}</Text>
          </li>
          <li>
            <Price price={harvest.price} />
          </li>
          <li>
            <Text>{harvest.type}</Text>
          </li>
          <li>
            <Text>{harvest.description}</Text>
          </li>
        </ul>
      </div>

      <div>
        <Subtitle1>Available at</Subtitle1>
        <ul>
          <li>
            <Text>{harvest.farm.name}</Text>
          </li>
          <li>
            <Text>{harvest.farm.contactNumber}</Text>
          </li>
          <li>
            <Text>{harvest.farm.contactEmail}</Text>
          </li>
          <li>
            <Text>{harvest.location.placeName}</Text>
          </li>
          <li>
            <Text>{harvest.location.region}</Text>
          </li>
          <li>
            <Text>{harvest.location.coords}</Text>
          </li>
        </ul>
      </div>
      <div>
        <Subtitle1>Farm owner</Subtitle1>
        <Persona
          onClick={toUserProfile}
          name={farmOwner.fullName}
          size="huge"
          avatar={{
            image: {
              src: farmOwner.photo && userPhotoResolver(farmOwner.photo),
            },
          }}
          {...props}
        />
      </div>
    </div>
  );
}
