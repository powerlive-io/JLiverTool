import defaultMdxComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';
import { Screenshot } from '@/components/screenshot';

export function getMDXComponents(): MDXComponents {
  return {
    ...defaultMdxComponents,
    Screenshot,
  };
}

export const useMDXComponents = getMDXComponents;
