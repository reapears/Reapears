import React, { useState } from "react";
import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
  Select,
  Textarea,
  Label,
} from "@fluentui/react-components";

import { DatePicker } from "@fluentui/react-datepicker-compat";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function HarvestCreate(props) {
  const styles = useStyles();
  const [harvest, setHarvest] = useState({
    locationId: "",
    cultivarId: "",
    price: { amount: null, unit: null },
    type: "",
    description: "",
    availableAt: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setHarvest((oldHarvest) => ({ ...oldHarvest, [key]: value }));
  };

  const onAvailableDateChange = (value) => {
    setHarvest((oldHarvest) => ({ ...oldHarvest, ["availableAt"]: value }));
  };

  const onPriceChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setHarvest((oldHarvest) => {
      oldHarvest["price"][key] = value;
      return { ...oldHarvest };
    });
  };

  const submitForm = (event) => {
    createHarvest(harvest);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Location" {...props}>
        <Select
          value={harvest.locationId}
          name="locationId"
          onChange={onChange}
          {...props}
        >
          <option value="namibia">Select location</option>
          <option value="location">User farm location here</option>
        </Select>
      </Field>

      <Field label="Cultivar" {...props}>
        <Select
          value={harvest.cultivarId}
          name="cultivarId"
          onChange={onChange}
          {...props}
        >
          <option value="">Select cultivar</option>
          <option value="tomato">Tomatoes</option>
          <option value="onion">Onions</option>
          <option value="bell pepper">Bell pepper</option>
        </Select>
      </Field>

      <Field label="Cultivar type" {...props}>
        <Input value={harvest.type} name="type" onChange={onChange} />
      </Field>

      <Field label="Harvest description" {...props}>
        <Textarea
          name="description"
          value={harvest.description}
          onChange={onChange}
          {...props}
        />
      </Field>

      <Field label="Available date" required>
        <DatePicker
          onSelectDate={onAvailableDateChange}
          placeholder="Select a date..."
          {...props}
        />
      </Field>   
    
      <div>
        <Label {...props}>Price</Label>
        <Field label="Amount" {...props} required>
          <Input
            contentBefore="N$"
            value={harvest.price.amount}
            name="amount"
            onChange={onPriceChange}
          />
        </Field>

        <Field label="" {...props} required>
          <Select
            value={harvest.price.unit}
            name="unit"
            onChange={onPriceChange}
            {...props}
          >
            <option value="">Select unit</option>
            <option value="kg">Kg</option>
            <option value="crate">Crate</option>
          </Select>
        </Field>
      </div>

      <Button appearance="primary" {...props}>
        List your harvest
      </Button>

      <pre>{JSON.stringify(harvest, true, 2)}</pre>
    </form>
  );
}

function createHarvest(harvest) {
  console.log(JSON.stringify(harvest));
}
