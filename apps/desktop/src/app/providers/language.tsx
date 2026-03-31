import { createContext, createSignal, onCleanup, type ParentProps, useContext } from "solid-js";
import { I18nextProvider } from "solid-i18next";

import { i18n, isAppLanguage, type AppLanguage } from "../../shared/i18n";

type LanguageContextValue = {
  language: () => AppLanguage;
  setLanguage: (value: AppLanguage) => Promise<void>;
};

const LanguageContext = createContext<LanguageContextValue>();

function resolveCurrentLanguage(): AppLanguage {
  const language = i18n.resolvedLanguage ?? i18n.language;
  return isAppLanguage(language) ? language : "ru";
}

export function LanguageProvider(props: ParentProps) {
  const [language, setLanguageState] = createSignal<AppLanguage>(resolveCurrentLanguage());

  const handleLanguageChanged = (nextLanguage: string) => {
    if (isAppLanguage(nextLanguage)) {
      setLanguageState(nextLanguage);
    }
  };

  i18n.on("languageChanged", handleLanguageChanged);
  onCleanup(() => i18n.off("languageChanged", handleLanguageChanged));

  const setLanguage = async (value: AppLanguage) => {
    if (value !== (i18n.resolvedLanguage ?? i18n.language)) {
      await i18n.changeLanguage(value);
    }
  };

  return (
    <I18nextProvider i18n={i18n}>
      <LanguageContext.Provider value={{ language, setLanguage }}>
        {props.children}
      </LanguageContext.Provider>
    </I18nextProvider>
  );
}

export function useLanguage() {
  const value = useContext(LanguageContext);
  if (!value) {
    throw new Error("useLanguage must be used within LanguageProvider");
  }
  return value;
}
