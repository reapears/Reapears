import { React, useState } from "react";
import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
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

export function FarmUpdate(props) {
  const styles = useStyles();
  const [farm, setFarm] = useState({
    name: "",
    contactNumber: "",
    contactEmail: "",
    foundedAt: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setFarm((oldFarm) => {
      return { ...oldFarm, [key]: value };
    });
  };

  const onFoundDateChange = (value) => {
    setFarm((oldFarm) => ({ ...oldFarm, ["foundedAt"]: value }));
  };

  const submitForm = (event) => {
    updateFarm(farm);
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
          value={farm.foundedAt}
          onSelectDate={onFoundDateChange}
          placeholder="Select a date..."
          {...props}
        />
      </Field>

      <Button appearance="primary" {...props}>
        Save
      </Button>

      <pre>{JSON.stringify(farm, true, 2)}</pre>
    </form>
  );
}

function updateFarm(farm) {
  console.log(JSON.stringify(farm));
}
