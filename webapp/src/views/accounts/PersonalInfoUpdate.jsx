import { React, useState } from "react";

import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
  Select,
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

export function PersonalInfoUpdate(props) {
  const styles = useStyles();
  const [user, setUser] = useState({
    firstName: "",
    lastName: "",
    gender: "",
    dateOfBirth: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const onDateChange = (value) => {
    setUser((oldUser) => ({ ...oldUser, ["dateOfBirth"]: value }));
  };

  const submitForm = (event) => {
    updatePersonalInfo(user);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="First name" required {...props}>
        <Input name="firstName" value={user.firstName} onChange={onChange} />
      </Field>

      <Field label="Last name" {...props}>
        <Input name="lastName" value={user.lastName} onChange={onChange} />
      </Field>

      <Field label="Gender" {...props}>
        <Select
          value={user.gender}
          name="gender"
          onChange={onChange}
          {...props}
        >
          <option value="male">Male</option>
          <option value="female">Female</option>
          <option value="">Not specified</option>
        </Select>
      </Field>

      <Field label="Date of birth">
        <DatePicker
          value={user.dateOfBirth}
          onSelectDate={onDateChange}
          placeholder="Select a date..."
          {...props}
        />
      </Field>

      <Button appearance="primary" {...props}>
        Save
      </Button>
      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function updatePersonalInfo(user) {
  console.log(JSON.stringify(user));
}
