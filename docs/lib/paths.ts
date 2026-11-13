// Next.js prefixes Link hrefs automatically, but public assets and fetch URLs do not.
export function withBasePath(path: string) {
  return `${process.env.NEXT_PUBLIC_BASE_PATH || ''}${path}`;
}
