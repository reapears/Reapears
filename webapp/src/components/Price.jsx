import React from "react";

import { Text, Caption1 } from "@fluentui/react-components";

export function Price({ price }) {
  return (
    <>
      <Text font="numeric">N${price.amount}.00</Text>
      <Caption1 italic size={300}>
        {getPriceUnit(price)}
      </Caption1>
    </>
  );
}

function getPriceUnit(price) {
  if (price.unit === "Crate") {
    return " per crate";
  }
  const nkg = price.unit.Kg;
  return ` per ${nkg}kg`;
}
