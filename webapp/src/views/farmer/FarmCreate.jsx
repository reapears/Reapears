import { React, useState } from "react";

import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
  Select,
  Textarea,
} from "@fluentui/react-components";

import { Location24Regular } from "@fluentui/react-icons";

import { DatePicker } from "@fluentui/react-datepicker-compat";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function FarmCreate(props) {
  const styles = useStyles();

  const [farm, setFarm] = useState({
    name: "",
    contactNumber: "",
    contactEmail: "",
    foundedAt: "",
    location: {
      countryId: "",
      regionId: "",
      placeName: "",
      description: "",
      coords: {},
    },
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setFarm((oldFarm) => ({ ...oldFarm, [key]: value }));
  };

  const onLocationChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setFarm((oldFarm) => {
      oldFarm["location"][key] = value;
      return { ...oldFarm };
    });
  };

  const onFoundDateChange = (value) => {
    setFarm((oldFarm) => ({ ...oldFarm, ["foundedAt"]: value }));
  };

  const onClickGeoPosition = () => {
    setFarm((oldFarm) => {
      oldFarm["location"]["coords"] = { x: 12.323, y: 4.343 };
      return { ...oldFarm };
    });
  };

  const submitForm = (event) => {
    createFarm(farm);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Farm name" required {...props}>
        <Input name="name" value={farm.name} onChange={onChange} />
      </Field>

      <Field label="Contact number" {...props}>
        <Input
          name="contactNumber"
          value={farm.contactNumber}
          onChange={onChange}
          type="phone"
        />
      </Field>

      <Field label="Contact email" {...props}>
        <Input
          name="contactEmail"
          value={farm.contactEmail}
          onChange={onChange}
          type="email"
        />
      </Field>

      <Field label="Date founded">
        <DatePicker
          onSelectDate={onFoundDateChange}
          placeholder="Select a date..."
          {...props}
        />
      </Field>

      <Field label="Country" {...props}>
        <Select
          value={farm.location.countryId}
          name="countryId"
          onChange={onLocationChange}
          {...props}
        >
          <option value="namibia">Select country</option>
          <option value="namibia">Namibia</option>
        </Select>
      </Field>

      <Field label="Region" {...props}>
        <Select
          value={farm.location.regionId}
          name="regionId"
          onChange={onLocationChange}
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
          value={farm.location.placeName}
          onChange={onLocationChange}
        />
      </Field>

      <Field label="Location description" {...props}>
        <Textarea
          name="description"
          value={farm.location.description}
          onChange={onLocationChange}
          {...props}
        />
      </Field>

      <Button onClick={onClickGeoPosition} icon={<Location24Regular />}>
        Add geo position
      </Button>

      <Button appearance="primary" {...props}>
        Create farm
      </Button>

      <pre>{JSON.stringify(farm, true, 2)}</pre>
    </form>
  );
}

function createFarm(farm) {
  console.log(JSON.stringify(farm));
}
