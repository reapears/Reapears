import { configureStore, createSlice } from "@reduxjs/toolkit";

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
