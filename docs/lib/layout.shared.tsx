import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { withBasePath } from './paths';

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: (
        <>
          <img src={withBasePath('/logo.png')} width={28} height={28} alt="" />
          <span className="font-semibold tracking-tight">JLiverTool</span>
        </>
      ),
    },
    githubUrl: 'https://github.com/powerlive-io/JLiverTool',
    links: [
      { text: '使用指南', url: '/setup', active: 'nested-url' },
      { text: '插件开发', url: '/plugin-document', active: 'nested-url' },
      { text: '更新笔记', url: '/changelog', active: 'nested-url' },
    ],
  };
}
