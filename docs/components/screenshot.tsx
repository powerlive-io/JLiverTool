'use client';

import { useEffect, useId, useRef, useState } from 'react';
import { Expand, X } from 'lucide-react';
import { withBasePath } from '@/lib/paths';

type ScreenshotProps = {
  src: string;
  alt: string;
  width: number;
  height: number;
  /** Source-image coordinates: x, y, width, height. The original stays intact. */
  crop?: [number, number, number, number];
  caption: string;
};

export function Screenshot({ src, alt, width, height, crop, caption }: ScreenshotProps) {
  const [open, setOpen] = useState(false);
  const dialog = useRef<HTMLDialogElement>(null);
  const titleId = useId();
  const url = withBasePath(src);

  useEffect(() => {
    if (!open) return;
    const element = dialog.current;
    element?.showModal();
    const previousOverflow = document.body.style.overflow;
    document.body.style.overflow = 'hidden';
    return () => {
      element?.close();
      document.body.style.overflow = previousOverflow;
    };
  }, [open]);

  return (
    <figure className="doc-screenshot not-prose">
      <button
        type="button"
        className="doc-screenshot-button"
        aria-label={`查看原图：${alt}`}
        aria-haspopup="dialog"
        onClick={() => setOpen(true)}
      >
        <span
          className={`doc-screenshot-preview${crop ? ' is-cropped' : ''}`}
          style={crop ? { aspectRatio: `${crop[2]} / ${crop[3]}` } : undefined}
        >
          <img
            src={url}
            alt={alt}
            width={width}
            height={height}
            loading="lazy"
            style={crop ? {
              width: `${width / crop[2] * 100}%`,
              left: `${-crop[0] / crop[2] * 100}%`,
              top: `${-crop[1] / crop[3] * 100}%`,
            } : undefined}
          />
        </span>
        <span className="doc-screenshot-action"><Expand size={13} aria-hidden="true" />查看原图</span>
      </button>
      <figcaption>{caption}</figcaption>
      <dialog
        ref={dialog}
        className="doc-screenshot-dialog"
        aria-labelledby={titleId}
        onClose={() => setOpen(false)}
        onClick={(event) => {
          if (event.target === event.currentTarget) dialog.current?.close();
        }}
      >
        <div className="doc-screenshot-viewer">
          <div className="doc-screenshot-toolbar">
            <span id={titleId}>{alt}</span>
            <a href={url} target="_blank" rel="noreferrer">新标签页查看</a>
            <button type="button" aria-label="关闭原图" onClick={() => dialog.current?.close()} autoFocus>
              <X size={20} aria-hidden="true" />
            </button>
          </div>
          <div className="doc-screenshot-original">
            <img src={url} alt={alt} width={width} height={height} />
          </div>
        </div>
      </dialog>
    </figure>
  );
}
