"use strict";

/**
 * Construct a url to the server api
 * @param {string} path url path and optional query parameters
 * @returns server(Reapears) api URL to the path provided
 */
function endpointUrl(path) {
  const url = new URL(`http://localhost:4000/api/v1/${path}`);
  url.searchParams.append("api_key", "efgryh5364rf34xfred");
  return url;
}

/**
 * Media routes on the server.
 */
const MEDIA_URL = {
  harvest: (id) => endpointUrl(`harvests/p/${id}`),
  cultivar: (id) => endpointUrl(`cultivars/p/${id}`),
  user: (id) => endpointUrl(`/account/users/photo${id}`),
};

/**
 *
 * Convert an object of filters into an url query string.
 * @param {object} filters
 * @returns url query string
 *
 *
 * Example:
 *
 * Input: {
 *  cultivar: ["tomatoes", "onion"],
 *  region: ["omusati", "ohangwena"],
 * }
 *
 * Output:  "cultivar=tomatoes&cultivar=onion&region=omusati&region=ohangwena"
 */
function intoUrlQuery(filters) {
  let query = "";
  for (let name in filters) {
    for (let value of filters[name]) {
      query += `${name}=${value}&`;
    }
  }
  // remove the ampersand(&) at the end
  return query.slice(0, query.length - 1);
}

function getHarvests(filters) {
  let query = intoUrlQuery(filters);
  query = query ? `?${query}` : "";
  const fetchHarvests = async () => {
    const { data } = await axios.get(endpointUrl(`produce${query}`));
    return data;
  };
  return useQuery("produce", fetchHarvests);
}

export { endpointUrl, MEDIA_URL };

// import { useQuery } from "react-query";
// import axios from "axios";

// const fetchUser = async () => {
// const { data } = await axios.get(
//   `https://jsonplaceholder.typicode.com/users/1`
// );
// return data;
// };
// const {
//        isLoading,
//        isSuccess,
//        error,
//        isError,
//        data: userData
//     } = useQuery("user",fetchUser);

// <div>
//   {isLoading && <article>...Loading user </article>}
//   {isError && <article>{error.message}</article>}
//   {isSuccess && (
//     <article>
//       <p>Username: {userData.username}</p>
//     </article>
//   )}
// </div>
