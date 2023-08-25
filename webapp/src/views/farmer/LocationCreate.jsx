import { React, useState } from "react";
import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
  Textarea,
  Select,
} from "@fluentui/react-components";
import { Location24Regular } from "@fluentui/react-icons";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function LocationCreate(props) {
  const styles = useStyles();
  const [location, setLocation] = useState({
    countryId: "",
    regionId: "",
    placeName: "",
    description: "",
    coords: {},
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setLocation((oldLocation) => ({ ...oldLocation, [key]: value }));
  };

  const onClickGeoPosition = () => {
    setLocation((oldLocation) => {
      oldLocation["coords"] = { x: 12.323, y: 4.343 };
      return { ...oldLocation };
    });
  };

  const submitForm = (event) => {
    createLocation(location);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Country" {...props}>
        <Select
          value={location.countryId}
          name="countryId"
          onChange={onChange}
          {...props}
        >
          <option value="namibia">Select country</option>
          <option value="namibia">Namibia</option>
        </Select>
      </Field>

      <Field label="Region" {...props}>
        <Select
          value={location.regionId}
          name="regionId"
          onChange={onChange}
          {...props}
        >
          <option value="">Select region</option>
          <option value="omusati">Omusati</option>
          <option value="ohangwena">Ohangwena</option>
          <option value="kavango west">Kavango West</option>
        </Select>
      </Field>

      <Field label="Place name" required {...props}>
        <Input
          name="placeName"
          value={location.placeName}
          onChange={onChange}
        />
      </Field>

      <Field label="Location description" {...props}>
        <Textarea
          name="description"
          value={location.description}
          onChange={onChange}
          {...props}
        />
      </Field>

      <Button onClick={onClickGeoPosition} icon={<Location24Regular />}>
        Add geo position
      </Button>

      <Button appearance="primary" {...props}>
        Add location
      </Button>

      <pre>{JSON.stringify(location, true, 2)}</pre>
    </form>
  );
}

function createLocation(location) {
  console.log(JSON.stringify(location));
}
