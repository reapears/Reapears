import React from "react";
import { useNavigate } from "react-router-dom";
import {
  makeStyles,
  shorthands,
  Caption1,
  Body1,
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
  title: {
    ...shorthands.margin(0, 0, "12px"),
  },

  description: {
    ...shorthands.margin(0, 0, "12px"),
  },

  card: {
    width: "250px",
    maxWidth: "100%",
    height: "fit-content",
    // height: "370px",
  },

  cardPreview: {
    height: "200px",
    // fit: "cover",
  },

  text: {
    ...shorthands.margin(0),
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
    <div>
      <Card
        onClick={openHarvestCard}
        size="large"
        className={styles.card}
        {...props}
      >
        <CardPreview
          className={styles.cardPreview}
          logo={
            <Avatar
              shape="square"
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
            src={previewImage(harvest)}
            alt={`${harvest.name} picture`}
            fit="cover"
          />
        </CardPreview>

        <CardHeader
          header={
            <Body1>
              <b>{harvest.name}</b>
            </Body1>
          }
          description={
            harvest.type && (
              <Badge appearance="tint" italic>
                {harvest.type}
              </Badge>
            )
          }
          // action={
          //   <Button
          //     appearance="transparent"
          //     icon={<MoreHorizontal20Filled />}
          //     aria-label="More options"
          //   />
          // }
        />

        <ul>
          <li>
            <Price price={harvest.price} />
          </li>

          <li>
            <Text size={400}>{harvest.placeName} </Text>
            <Location12Regular />
          </li>
          <li>
            <Caption1>{harvest.region}</Caption1>
          </li>

          <li>
            <Body1>{toDisplayDate(harvest.harvestDate)}</Body1>
          </li>
        </ul>
      </Card>
    </div>
  );
}

// ===== Util functions impls =====

function previewImage(harvest) {
  if (harvest.images) {
    return harvestImageResolver(harvest.images[0]);
  }
  // harvest.cultivarImage
  return cultivarImageResolver("cultivar-default.jpg");
}

function toDisplayDate(date) {
  const harvestDate = new Date(date);
  const prefix =
    Date.now() > harvestDate ? "Harvesting started" : "Harvesting start";
  const options = { year: "numeric", month: "short", day: "numeric" };
  const localeDate = harvestDate.toLocaleDateString(undefined, options);
  return `${prefix} - ${localeDate} `;
}
