import defaultMdxComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';
import { withBasePath } from './lib/paths';

export function getMDXComponents(): MDXComponents {
  return {
    ...defaultMdxComponents,
    img: ({ src, alt, ...props }) => (
      <img
        {...props}
        src={typeof src === 'string' && src.startsWith('/') ? withBasePath(src) : src}
        alt={alt ?? ''}
        loading="lazy"
      />
    ),
  };
}

export const useMDXComponents = getMDXComponents;
