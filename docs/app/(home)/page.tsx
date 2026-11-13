import Link from 'next/link';
import { ArrowUpRight, BookOpen, Download, Puzzle, Radio } from 'lucide-react';
import { withBasePath } from '@/lib/paths';
import { LiveDemo } from '@/components/live-demo';

const guides = [
  { href: '/setup', icon: Radio, title: '连接你的直播间', text: '下载安装、扫码登录，从第一条弹幕开始。' },
  { href: '/user-document', icon: BookOpen, title: '熟悉每一个窗口', text: '弹幕、礼物、醒目留言与观众记录，各有其位。' },
  { href: '/plugin-document', icon: Puzzle, title: '让插件加入直播', text: '安装现有插件，或用 jliverAPI 实现自己的想法。' },
];

export default function HomePage() {
  return (
    <main className="home-shell">
      <section className="home-hero">
        <div className="hero-copy">
          <img className="hero-logo" src={withBasePath('/logo.png')} alt="JLiverTool" width={64} height={64} />
          <h1>你的直播间，<br />每一条弹幕都在这里。</h1>
          <p className="hero-description">JLiverTool 是简单易用的 B 站开源直播弹幕工具。把弹幕、礼物和醒目留言放在手边，专注和观众交流。</p>
          <div className="hero-actions">
            <Link className="home-button primary" href="/setup">开始使用</Link>
            <a className="home-button secondary" href="https://github.com/powerlive-io/JLiverTool/releases"><Download size={17} />下载应用</a>
          </div>
          <p className="platforms">适用于 Windows、macOS 和 Linux</p>
        </div>
        <LiveDemo />
      </section>

      <section className="guide-section" aria-labelledby="guide-heading">
        <div className="section-intro">
          <h2 id="guide-heading">从这里开始</h2>
          <p>第一次使用，或准备做点新东西。</p>
        </div>
        <div className="guide-list">
          {guides.map(({ href, icon: Icon, title, text }) => (
            <Link className="guide-link" href={href} key={href}>
              <Icon className="guide-icon" size={23} />
              <div><h3>{title}</h3><p>{text}</p></div>
              <ArrowUpRight className="guide-arrow" size={19} />
            </Link>
          ))}
        </div>
      </section>

      <section className="home-details" aria-label="更多文档">
        <div><h2>按自己的习惯使用</h2><p>调整窗口主题、透明度与字体，开启语音播报，或通过插件接收直播事件。让工具适应你的直播方式。</p><Link href="/user-document">查看功能文档</Link></div>
        <div><h2>遇到问题？</h2><p>先查阅安装与连接的常见问题。需要反馈或有新想法，也欢迎到 GitHub 与我们交流。</p><Link href="/qa">查看常见问题</Link></div>
      </section>
      <footer className="home-footer"><span>JLiverTool · 开源直播弹幕工具</span><div><Link href="/plugin-list">插件列表</Link><Link href="/changelog">更新笔记</Link><a href="https://github.com/powerlive-io/JLiverTool">GitHub</a></div></footer>
    </main>
  );
}
