import React from "react";

import {
  Field,
  shorthands,
  makeStyles,
  Button,
} from "@fluentui/react-components";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "200px",
  },
});

export function UserProfilePhoto(props) {
  const styles = useStyles();
  return (
    <form
      action="/"
      method="post"
      encType="multipart/form-data"
      className={styles.root}
    >
      <Field label="Upload profile photo" {...props}>
        <input type="file" name="profile-photo" accept=".jpg, .jpeg, .png" />
      </Field>

      <Button type="submit" appearance="primary" {...props}>
        Upload photo
      </Button>
    </form>
  );
}

{
  /* <input type="file" name="profile-photo" accept="image/png, image/jpeg" multiple /> */
}
{
  /* <input type="file" id="avatar" name="avatar" accept="image/png, image/jpeg" /> */
}
