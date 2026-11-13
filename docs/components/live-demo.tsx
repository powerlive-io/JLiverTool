'use client';

import { useEffect, useRef, useState, type CSSProperties, type FormEvent, type KeyboardEvent } from 'react';
import { withBasePath } from '@/lib/paths';
import './live-demo.css';

type IconKind = 'pin' | 'dashboard' | 'gift' | 'sc' | 'stats' | 'audience' | 'settings';
type Message = { name: string; text: string; level?: number; guard?: 2 | 3 };

// Layout and dimensions follow main_view/render.rs and danmu_list_item.rs.
// Colors follow ThemeColors::dracula() and Colors::medal_colors() in theme.rs.
const initialMessages: Message[] = [
  { name: '小岛来信', text: '晚上好，今天也来听歌', level: 27 },
  { name: '北极星', text: '这个声音好温柔', level: 22, guard: 3 },
  { name: '晚风', text: '来了来了', level: 26 },
  { name: '深蓝月光', text: '今天的第一首就很好听', level: 21 },
  { name: '认真听歌的猫', text: '👏👏', level: 35, guard: 3 },
  { name: '米汀的小笼包', text: '好听！', level: 47, guard: 2 },
  { name: 'Hana_夏', text: '这首歌我也循环了好久' },
  { name: 'soulのtouch', text: '今天声音好像有一点点哑，唱的很有感觉', level: 45, guard: 2 },
  { name: '星星不说话', text: '耳机已经戴好了', level: 49, guard: 2 },
  { name: '一杯冰美式', text: '下班路上正好赶上直播', level: 36, guard: 3 },
  { name: 'elecRui', text: '是有一点', level: 29 },
  { name: '童话避难所', text: '慢慢唱，我们都在', level: 32, guard: 3 },
  { name: '月饼晴朗', text: '带了一些了', level: 47 },
  { name: '夜梦YuMe', text: '晚上的电台时间 🎵', level: 33, guard: 3 },
  { name: '白桃汽水', text: '再来一首吧！', level: 18 },
];
const commands = [
  { command: '/title', description: '修改直播间标题' },
  { command: '/bye', description: '关闭直播' },
];

function NativeIcon({ kind }: { kind: IconKind }) {
  const shapes = {
    pin: <><circle className="native-pin-head" cx="7" cy="4" r="3.5" /><path d="M7 8v6" strokeWidth="2" /></>,
    dashboard: <><rect x="0.5" y="0.5" width="5" height="5" rx="0.5" /><rect x="8.5" y="0.5" width="5" height="5" rx="0.5" /><rect x="0.5" y="8.5" width="5" height="5" rx="0.5" /><rect x="8.5" y="8.5" width="5" height="5" rx="0.5" /></>,
    gift: <><path d="M1 5h12v8H1zM0 2h14v3H0z" /><path d="M7 5v8" strokeWidth="2" /></>,
    sc: <path d="M3 .5h8a2.5 2.5 0 0 1 2.5 2.5v8a2.5 2.5 0 0 1-2.5 2.5H.5V3A2.5 2.5 0 0 1 3 .5Z" />,
    stats: <g fill="currentColor" stroke="none"><path d="M1 8h3v6H1zM5 4h3v10H5zM9 6h3v8H9z" /></g>,
    audience: <g fill="currentColor" stroke="none"><circle cx="3.5" cy="5" r="2" /><circle cx="10.5" cy="5" r="2" /><path d="M.5 13V10a3 3 0 0 1 6 0v3zm7 0V10a3 3 0 0 1 6 0v3z" /></g>,
    settings: <path d="M1 3h12M1 7h12M1 11h12" strokeWidth="2" strokeLinecap="round" />,
  };
  return <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" aria-hidden="true">{shapes[kind]}</svg>;
}

function Medal({ level, guard }: { level: number; guard?: 2 | 3 }) {
  const [color, border] = level <= 10 ? ['hsl(230 32% 50% / .6)', 'hsl(230 32% 50% / .6)']
    : level <= 20 ? ['hsl(320 44% 61% / .6)', 'hsl(320 44% 61% / .6)']
    : level <= 30 ? ['hsl(200 91% 61% / .6)', 'hsl(200 91% 61% / .6)']
    : level <= 40 ? ['hsl(222 100% 65% / .6)', 'hsl(222 100% 65% / .6)']
    : ['hsl(270 78% 70% / .6)', 'hsl(285 100% 74%)'];
  return <span className="native-medal" style={{ '--medal': color, '--medal-border': border, '--medal-text': color.replace(' / .6', '') } as CSSProperties}>
    <span className="native-medal-name">{guard && <img src={withBasePath(`/showcase/guard-${guard}.webp`)} width="15" height="15" alt={guard === 3 ? '舰长' : '提督'} />}小电台</span><span className="native-medal-level">{level}</span>
  </span>;
}

export function LiveDemo() {
  const [messages, setMessages] = useState(initialMessages);
  const [input, setInput] = useState('');
  const [title, setTitle] = useState('晚间电台 · 定好行程练练歌');
  const [live, setLive] = useState(true);
  const [pinned, setPinned] = useState(true);
  const [dismissed, setDismissed] = useState(false);
  const [selected, setSelected] = useState(0);
  const feed = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const choices = commands.filter(({ command }) => command.startsWith(input));
  const showCommands = !dismissed && input.startsWith('/') && !input.includes(' ') && choices.length > 0;

  useEffect(() => { if (messages !== initialMessages) feed.current?.scrollTo({ top: feed.current.scrollHeight }); }, [messages]);

  function complete(command: string) {
    setInput(command === '/title' ? '/title ' : command);
    setDismissed(true);
    inputRef.current?.focus();
  }

  function send(event: FormEvent) {
    event.preventDefault();
    const text = input.trim();
    if (!text) return;
    if (text.startsWith('/title ') && text.slice(7).trim()) setTitle(text.slice(7).trim());
    else if (text === '/bye') setLive(false);
    else setMessages(previous => [...previous, { name: '你', text, level: 21 }].slice(-50));
    setInput('');
    setDismissed(false);
  }

  function onKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.nativeEvent.isComposing) { if (event.key === 'Enter') event.preventDefault(); return; }
    if (!showCommands) return;
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      setSelected((selected + (event.key === 'ArrowDown' ? 1 : choices.length - 1)) % choices.length);
    } else if (event.key === 'Enter' || event.key === 'Tab') {
      event.preventDefault(); complete(choices[selected]?.command ?? choices[0].command);
    } else if (event.key === 'Escape') setDismissed(true);
  }

  function reset() {
    setMessages(initialMessages); setInput(''); setTitle('晚间电台 · 定好行程练练歌');
    setLive(true); setPinned(true); setDismissed(false); setSelected(0);
    feed.current?.scrollTo({ top: 0 });
  }

  return (
    <figure className="hero-demo">
      <div className="native-window-viewport">
        <section className="native-window" aria-label="JLiverTool 主窗口展示" aria-describedby="demo-caption">
          <header className={`native-titlebar ${live ? 'is-live' : ''}`}>
            <div className="native-traffic-lights" aria-hidden="true"><i /><i /><i /></div>
            <div className="native-live-status">{live && <><strong>LIVE</strong><span>340</span></>}</div>
            <div className="native-toolbar" role="group" aria-label="主窗口工具栏">
              <button type="button" className="native-tool" title="置顶" aria-label="置顶" aria-pressed={pinned} onClick={() => setPinned(!pinned)}><NativeIcon kind="pin" /></button>
              <span className="native-tool native-tool-static" title="仪表盘模式（桌面端功能）" role="img" aria-label="仪表盘模式"><NativeIcon kind="dashboard" /></span>
              {([
                ['gift', '礼物窗口'],
                ['sc', '醒目留言窗口'],
                ['stats', '统计窗口'],
                ['audience', '观众列表'],
                ['settings', '设置'],
              ] as const).map(([kind, label]) => <span key={kind} title={label} aria-label={label} role="img" className="native-tool native-tool-static"><NativeIcon kind={kind} /></span>)}
            </div>
          </header>
          <div className="native-feed" ref={feed} role="log" aria-label="模拟弹幕" aria-live="polite">
            {messages.map((message, i) => <div className="native-message" key={i}>
              {message.level && <Medal level={message.level} guard={message.guard} />}
              <span className="native-username">{message.name}:</span><span className="native-message-text">{' '}{message.text}</span>
            </div>)}
          </div>
          <footer className="native-footer">
            <div className="native-room-status"><span>{title}</span><i title="已连接" aria-label="已连接" /></div>
            <form className="native-composer" onSubmit={send}>
              {showCommands && <div className="native-commands" id="demo-commands" role="listbox" aria-label="弹幕命令">
                {choices.map(({ command, description }, i) => <button type="button" role="option" id={`demo-command-${i}`} aria-selected={selected === i} key={command} onMouseDown={event => event.preventDefault()} onClick={() => complete(command)}><span>{command}</span>{description}</button>)}
              </div>}
              <div className="native-input-wrap">
                <input ref={inputRef} value={input} maxLength={100} placeholder="发送弹幕或输入命令 (/title, /bye)..." aria-label="发送模拟弹幕" role="combobox" aria-autocomplete="list" aria-expanded={showCommands} aria-controls={showCommands ? 'demo-commands' : undefined} aria-activedescendant={showCommands ? `demo-command-${selected}` : undefined} autoComplete="off" onKeyDown={onKeyDown} onBlur={() => setDismissed(true)} onChange={event => { setInput(event.target.value); setDismissed(false); setSelected(0); }} />
                {input && <button className="native-clear" type="button" aria-label="清空输入" onClick={() => { setInput(''); inputRef.current?.focus(); }}>×</button>}
              </div>
              <button className="native-send" type="submit">发送</button>
            </form>
          </footer>
        </section>
      </div>
      <figcaption id="demo-caption"><span>主窗口 · 模拟数据，可试用弹幕输入与命令</span><button type="button" onClick={reset}>重置展示</button></figcaption>
    </figure>
  );
}
