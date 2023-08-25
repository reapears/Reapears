import { React, useState } from "react";

import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
  Textarea,
} from "@fluentui/react-components";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function UserProfileUpdate(props) {
  const styles = useStyles();
  const [user, setUser] = useState({
    about: "",
    livesAt: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    updateUserProfile(user);
    event.preventDefault();
  };
  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="about" {...props}>
        <Textarea
          name="about"
          value={user.about}
          onChange={onChange}
          {...props}
        />
      </Field>

      <Field label="Lives at" {...props}>
        <Input name="livesAt" value={user.livesAt} onChange={onChange} />
      </Field>

      <Button appearance="primary" {...props}>
        Save
      </Button>
      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function updateUserProfile(user) {
  console.log(JSON.stringify(user));
}
