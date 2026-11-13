import { createMDX } from 'fumadocs-mdx/next';

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
  output: 'export',
  // This directory is also the MDX source; don't generate a non-page AGENTS.md here.
  agentRules: false,
  // Leave unset for local development; GitHub Pages supplies its repository path.
  basePath: process.env.NEXT_PUBLIC_BASE_PATH || '',
  images: { unoptimized: true },
  reactStrictMode: true,
};

export default withMDX(config);
