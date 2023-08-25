import React from "react";

import { useSearchParams } from "react-router-dom";

export function AccountConfirm() {
  const [query] = useSearchParams();
  const token = query.get("token")[0];
  return <div>AccountConfirm: token={token}</div>;
}

// import { useParams } from "react-router-dom";
// const params = useParams();
// params.id
