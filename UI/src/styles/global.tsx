import { createGlobalStyle, ThemeProvider, DefaultTheme } from 'styled-components';
import { PropsWithChildren } from 'react';

const GlobalStyle = createGlobalStyle`
  body {
    margin: 0; 
    padding: 0; 
    border: 0;
    background-color: #ffecaeff;
  
  }

  html {
    height: 100svh;
    scroll-behavior: smooth;
  }

  #root {
    font-family: 'Roboto', sans-serif;
    line-height: 1.5;
    font-weight: 400;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    height: 100vh;
  }

  ul {
    padding: 0;
    list-style-type: none;
    margin: 0;
  }

  *, *::after, *::before {
    box-sizing: border-box;
  }

  input, select, div {
    box-sizing: border-box;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
  }

  h1 { font-size: 2rem; }
  h2 { font-size: 1.5rem; }
  h3 { font-size: 1.17rem; }
  h4 { font-size: 1.00rem; }
  h5 { font-size: 0.83rem; }
  h6 { font-size: 0.67rem; }

  img, video, canvas {
    height: auto;
  }

  h1, h2, h3, h4, p, a, span, li {
    color: black;
    letter-spacing: 0.1em;
  }

  h1, h2, h3 {
    line-height: 1.3;
  }

  span, p {
    line-height: 1.75;
  }

  img {
    border-style: none;
  }

  h1 {
    font-size: clamp(1.2rem, -0.875rem + 8.333vw, 2.1rem);
  }

  h2, h3, a, li {
    font-size: clamp(1.2rem, -0.875rem + 8.333vw, 1.5rem);
  }

  p {
    font-size: clamp(1rem, -0.875rem + 1rem, 1.5rem);
  }

  a {
//todo
  }

  @media (min-width: 1024px) {
    a:hover::after {
      height: calc(100% + 0.5rem);
    }
  }
`;

const breakpoints = {
  xs: '320px',
  sm: '640px',
  md: '768px',
  lg: '1024px',
  xl: '1280px',
  '2xl': '1536px',
};

const theme: DefaultTheme = {
  devices: {
    xs: `(min-width: ${breakpoints.xs})`,
    sm: `(min-width: ${breakpoints.sm})`,
    md: `(min-width: ${breakpoints.md})`,
    lg: `(min-width: ${breakpoints.lg})`,
    xl: `(min-width: ${breakpoints.xl})`,
  },
  colors: {
    tangelo: '#f15620ff',
    vanilla: '#ffecaeff',
    jet: '#333333ff',
    coral: '#ff8254ff',
    viridian: '#55917fff',
  },
};

const Theme = ({ children }: PropsWithChildren<Record<string, unknown>>) => (
  <ThemeProvider theme={theme}>{children}</ThemeProvider>
);

export { GlobalStyle, Theme };

// /* SCSS Gradient */
// $gradient-top: linear-gradient(0deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-right: linear-gradient(90deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-bottom: linear-gradient(180deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-left: linear-gradient(270deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-top-right: linear-gradient(45deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-bottom-right: linear-gradient(135deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-top-left: linear-gradient(225deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-bottom-left: linear-gradient(315deg, #f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
// $gradient-radial: radial-gradient(#f15620ff, #ffecaeff, #333333ff, #ff8254ff, #55917fff);
