
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


// ======== Dark greeen theme

const greenTheme: BrandVariants = { 
  10: "#040401",
  20: "#1B1A09",
  30: "#2B2C0F",
  40: "#373910",
  50: "#434610",
  60: "#4F5310",
  70: "#5C610F",
  80: "#696F0D",
  90: "#767D08",
  100: "#848B02",
  110: "#939927",
  120: "#A4A749",
  130: "#B4B567",
  140: "#C3C384",
  150: "#D2D2A2",
  160: "#E1E0C0"
  };
  
   const lightGreenTheme: Theme = {
     ...createLightTheme(greenTheme), 
  };
  
   const darkGreenTheme: Theme = {
     ...createDarkTheme(greenTheme), 
  };
   
  
 darkGreenTheme.colorBrandForeground1 = greenTheme[110];
 darkGreenTheme.colorBrandForeground2 = greenTheme[120];