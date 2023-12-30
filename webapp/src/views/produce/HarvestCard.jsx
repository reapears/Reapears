import React from "react";
import { useNavigate } from "react-router-dom";
import {
  makeStyles,
  shorthands,
  Image,
  Badge,
  Text,
  Card,
  CardHeader,
  CardPreview,
  Avatar,
} from "@fluentui/react-components";
import { Location12Regular } from "@fluentui/react-icons";

import { Price } from "../../components";
import {
  cultivarImageResolver,
  farmLogoResolver,
  harvestImageResolver,
} from "../../endpoint";

const useStyles = makeStyles({
  card: {
    maxWidth: "100%",
    height: "fit-content",
    ...shorthands.borderRadius("14px"),
  },

  cardPreviewImage: {
    height: "24rem",
    ...shorthands.borderRadius("14px"),
  },

  cardPreview: {
    height: "24rem",
    fit: "cover",
    ...shorthands.borderRadius("14px"),
  },
});

export function HarvestCard(props) {
  const { harvest } = props;
  const styles = useStyles();
  const navigate = useNavigate();

  function openHarvestCard() {
    return navigate(`produce/${harvest.id}`);
  }

  return (
    <Card
      appearance="subtle"
      onClick={openHarvestCard}
      className={styles.card}
      {...props}
    >
      <CardPreview
        className={styles.cardPreview}
        logo={
          <Avatar
            shape="square"
            active="active"
            activeAppearance="ring-shadow"
            size={32}
            color="brand"
            name={harvest.farmName}
            image={{
              src: harvest.farmLogo && farmLogoResolver(harvest.farmLogo),
            }}
          />
        }
      >
        <Image
          className={styles.cardPreviewImage}
          src={previewImage(harvest)}
          alt={`${harvest.name} picture`}
          fit="cover"
        />
      </CardPreview>

      <CardHeader
        header={
          <Text size={400} weight="semibold">
            {harvest.name}
          </Text>
        }
        description={
          harvest.type ? (
            <Badge appearance="tint" italic>
              {harvest.type.toLowerCase()}
            </Badge>
          ) : (
            <Badge appearance="tint" italic>
              {harvest.category.toLowerCase()}
            </Badge>
          )
        }
      />

      <ul className="harvest-card-inner-list">
        <li>
          <Price price={harvest.price} />
        </li>

        <li>
          <Text>{toHarvestLocation(harvest)} </Text>
          <Location12Regular />
        </li>

        <li>
          <Text>{toDisplayDate(harvest.harvestDate)}</Text>
        </li>
      </ul>
    </Card>
  );
}

// ===== Util functions impls =====

function previewImage(harvest) {
  if (harvest.name === "Watermelon") {
    return cultivarImageResolver("watermelons-1.jpg");
  }

  if (harvest.name === "Onion") {
    return cultivarImageResolver("Onions-1.jpg");
  }
  if (harvest.name === "Butternuts") {
    return cultivarImageResolver("butternuts-1.jpg");
  }

  if (harvest.name === "Mango") {
    return cultivarImageResolver("magoes-1.jpg");
  }
  if (harvest.images) {
    return harvestImageResolver(harvest.images[0]);
  }
  // harvest.cultivarImage
  return cultivarImageResolver("cultivar-default.jpg");
}

function toHarvestLocation(harvest) {
  return `${harvest.placeName}, ${harvest.region}`;
}

function toDisplayDate(date) {
  const harvestDate = new Date(date);
  const prefix = Date.now() > harvestDate ? "Harvest began" : "Harvest begin";
  const options = { year: "numeric", month: "short", day: "numeric" };
  const localeDate = harvestDate.toLocaleDateString(undefined, options);
  return `${prefix} - ${localeDate}`;
}
