import { configureStore, createSlice } from "@reduxjs/toolkit";

const defaultUser = {
  loggedIn: false,
  profile: null,
  personalInfo: null,
  farms: null,
};

// getUserFarms, getUserLocations, getUserHarvests

const defaultUsher = {
  loggedIn: false,
  profile: {
    about: null,
    livesAt: null,
  },
  personalInfo: {
    firstName: null,
    lastName: null,
    gender: null,
    email: null,
  },
  farms: {
    farm: {
      name: null,
      contactEmail: null,
      contactNumber: null,
      logo: null,
      foundedAt: null,
      locations: [],
    },

    location: {
      placeName: null,
      region: null,
      country: null,
      coords: null,
      harvests: {},
    },
  },
};

const demoSlice = createSlice({
  name: "Demo",
  initialState: {
    demoList: [],
  },
  reducers: {
    add(state, action) {
      const newList = state.demoList.concat(action.payload);
      return { ...state, demoList: newList };
    },
    remove(state, action) {
      const newList = state.demoList.filter((i) => i.id !== action.payload.id);
      return { ...state, demoList: newList };
    },
  },
});

export const { add, remove } = demoSlice.actions;
const demoReducer = demoSlice.reducer;

export const store = configureStore({
  reducer: { demoReducer: demoReducer },
});
