import { createContext, createEffect, createSignal, type ParentProps, useContext } from "solid-js";

export type AppTheme = "dark" | "light";

type ThemeContextValue = {
  theme: () => AppTheme;
  setTheme: (value: AppTheme | ((current: AppTheme) => AppTheme)) => void;
};

const ThemeContext = createContext<ThemeContextValue>();

export function ThemeProvider(props: ParentProps) {
  const [theme, setTheme] = createSignal<AppTheme>("dark");

  createEffect(() => {
    const activeTheme = theme();
    document.documentElement.dataset.theme = activeTheme;
    localStorage.setItem("jb-skill.theme", activeTheme);
  });

  createEffect(() => {
    const savedTheme = localStorage.getItem("jb-skill.theme");
    if (savedTheme === "dark" || savedTheme === "light") {
      setTheme(savedTheme);
    }
  });

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {props.children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  const value = useContext(ThemeContext);
  if (!value) {
    throw new Error("useTheme must be used within ThemeProvider");
  }
  return value;
}
