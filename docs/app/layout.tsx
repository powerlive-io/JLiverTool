import type { Metadata } from 'next';
import type { ReactNode } from 'react';
import { Provider } from '@/components/provider';
import { withBasePath } from '@/lib/paths';
import './global.css';

export const metadata: Metadata = {
  title: { default: 'JLiverTool · B 站直播弹幕工具', template: '%s | JLiverTool' },
  description: 'JLiverTool 使用指南与插件开发文档。连接 B 站直播间，查看弹幕、礼物和醒目留言，用插件扩展直播体验。',
  icons: { icon: withBasePath('/logo.png') },
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="zh-CN" suppressHydrationWarning>
      <head>
        {/* GitHub Pages also serves /setup.html. Normalize before React reads the
            pathname so Fumadocs resolves the same page on server and client. */}
        <script dangerouslySetInnerHTML={{ __html: `
          if (location.pathname.endsWith('.html') && !location.pathname.endsWith('/404.html') && !location.pathname.endsWith('/_not-found.html')) {
            const path = location.pathname.replace(/\\/index\\.html$/, '/').replace(/\\.html$/, '');
            history.replaceState(history.state, '', path + location.search + location.hash);
          }
        ` }} />
      </head>
      <body className="flex min-h-screen flex-col">
        <Provider>{children}</Provider>
      </body>
    </html>
  );
}
