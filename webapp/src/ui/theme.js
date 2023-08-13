import { createLightTheme, createDarkTheme } from "@fluentui/react-components";

const brownTheme = {
  10: "#050300",
  20: "#201702",
  30: "#342502",
  40: "#443000",
  50: "#513C10",
  60: "#5E4921",
  70: "#6A5631",
  80: "#776341",
  90: "#847152",
  100: "#917F63",
  110: "#9E8E74",
  120: "#AB9C86",
  130: "#B8AB98",
  140: "#C5BBAB",
  150: "#D3CABE",
  160: "#E0DAD1",
};

const lightBrownTheme = {
  ...createLightTheme(brownTheme),
};

const darkBrownTheme = {
  ...createDarkTheme(brownTheme),
};

darkBrownTheme.colorBrandForeground1 = brownTheme[110];
darkBrownTheme.colorBrandForeground2 = brownTheme[120];

export { brownTheme, lightBrownTheme, darkBrownTheme };
