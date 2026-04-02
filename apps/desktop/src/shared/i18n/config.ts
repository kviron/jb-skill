import i18n from "i18next";

import commonEn from "./locales/en/common.json";
import navigationEn from "./locales/en/navigation.json";
import pagesEn from "./locales/en/pages.json";
import settingsEn from "./locales/en/settings.json";
import commonRu from "./locales/ru/common.json";
import navigationRu from "./locales/ru/navigation.json";
import pagesRu from "./locales/ru/pages.json";
import settingsRu from "./locales/ru/settings.json";

export type AppLanguage = "ru" | "en";

const LANGUAGE_STORAGE_KEY = "pantheon.language";
const DEFAULT_LANGUAGE: AppLanguage = "ru";

const resources = {
  ru: {
    common: commonRu,
    navigation: navigationRu,
    settings: settingsRu,
    pages: pagesRu,
  },
  en: {
    common: commonEn,
    navigation: navigationEn,
    settings: settingsEn,
    pages: pagesEn,
  },
} as const;

const namespaces = ["common", "navigation", "settings", "pages"] as const;

export function isAppLanguage(value: string | null | undefined): value is AppLanguage {
  return value === "ru" || value === "en";
}

function detectInitialLanguage(): AppLanguage {
  if (typeof window === "undefined") {
    return DEFAULT_LANGUAGE;
  }

  let storedLanguage = localStorage.getItem(LANGUAGE_STORAGE_KEY);
  if (!storedLanguage) {
    const legacy = localStorage.getItem("jb-skill.language");
    if (isAppLanguage(legacy)) {
      localStorage.setItem(LANGUAGE_STORAGE_KEY, legacy);
      storedLanguage = legacy;
    }
  }
  if (isAppLanguage(storedLanguage)) {
    return storedLanguage;
  }

  const browserLanguage = navigator.language.toLowerCase();
  if (browserLanguage.startsWith("en")) {
    return "en";
  }

  return DEFAULT_LANGUAGE;
}

const initialLanguage = detectInitialLanguage();

void i18n.init({
  resources,
  lng: initialLanguage,
  fallbackLng: DEFAULT_LANGUAGE,
  supportedLngs: ["ru", "en"],
  defaultNS: "common",
  ns: namespaces,
  interpolation: {
    escapeValue: false,
  },
});

if (typeof document !== "undefined") {
  document.documentElement.lang = initialLanguage;
}

i18n.on("languageChanged", (language) => {
  if (!isAppLanguage(language)) {
    return;
  }

  if (typeof window !== "undefined") {
    localStorage.setItem(LANGUAGE_STORAGE_KEY, language);
  }

  if (typeof document !== "undefined") {
    document.documentElement.lang = language;
  }
});

export { i18n, LANGUAGE_STORAGE_KEY };
