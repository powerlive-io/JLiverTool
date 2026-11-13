import type { Metadata } from 'next';
import { notFound } from 'next/navigation';
import { DocsBody, DocsDescription, DocsPage, DocsTitle } from 'fumadocs-ui/page';
import { source } from '@/lib/source';
import { getMDXComponents } from '@/mdx-components';

type Props = { params: Promise<{ slug: string }> };

export const dynamicParams = false;

export function generateStaticParams() {
  return source.getPages().map((page) => ({ slug: page.slugs[0] }));
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { slug } = await params;
  const page = source.getPage([slug]);
  if (!page) notFound();
  return { title: page.data.title, description: page.data.description };
}

export default async function Page({ params }: Props) {
  const { slug } = await params;
  const page = source.getPage([slug]);
  if (!page) notFound();
  const MDX = page.data.body;

  return (
    <DocsPage
      toc={page.data.toc}
      tableOfContent={{ style: 'clerk' }}
      editOnGithub={{
        owner: 'powerlive-io',
        repo: 'JLiverTool',
        sha: 'master',
        path: `docs/${page.path}`,
      }}
    >
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription>{page.data.description}</DocsDescription>
      <DocsBody><MDX components={getMDXComponents()} /></DocsBody>
    </DocsPage>
  );
}
