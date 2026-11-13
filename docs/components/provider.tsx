'use client';

import type { ReactNode } from 'react';
import { RootProvider } from 'fumadocs-ui/provider/next';
import { defineTranslations } from 'fumadocs-core/i18n';
import { i18nProvider, uiTranslations } from 'fumadocs-ui/i18n';
import { zhCN } from '@fumadocs/language/zh-cn';
import StaticSearchDialog from './search';

const translations = defineTranslations().extend(uiTranslations()).preset(zhCN());

export function Provider({ children }: { children: ReactNode }) {
  return (
    <RootProvider
      i18n={i18nProvider(translations)}
      search={{ SearchDialog: StaticSearchDialog }}
    >
      {children}
    </RootProvider>
  );
}
