# -*- coding: utf-8 -*-
"""渲染封面(模板01 HUD)并与 body.pdf 合并为最终交付 PDF。"""
import os
import sys

PDF_SKILL_DIR = r"C:\Users\Lenovo\.zcode\cli\plugins\cache\zcode-plugins-official\pdf\0.1.7\skills\pdf"
sys.path.insert(0, os.path.join(PDF_SKILL_DIR, "scripts"))

from cover_render import render_cover, detect_fonts

HERE = os.path.dirname(os.path.abspath(__file__))

content = {
    "kicker": "基于 RUSTLINGS 题单整理 · RUST STUDY NOTES",
    "hero": "Rust学习笔记",
    "summary": ("本笔记依据 Rustlings 题单系统整理 Rust 核心知识点："
                "从变量、所有权与借用，到泛型、trait 与生命周期，"
                "再到迭代器、智能指针、并发、宏与 unsafe/FFI；"
                "并覆盖十道数据结构与算法强化题的完整思路解析。"),
    "meta": "涵盖 28 个专题 · 16 章笔记 · 3 个附录",
    "footer": "RUSTLINGS KNOWLEDGE COMPANION",
    "year": "2026",
    "word": "RUST",
}

palette = {
    "primary": "#1f7692",   # ACCENT  (palette.generate, seed=42)
    "text": "#1b1a18",      # TEXT_PRIMARY
    "muted": "#7a766f",     # TEXT_MUTED
    "bg": "#ffffff",
}

fonts = detect_fonts()
cover_pdf = os.path.join(HERE, "cover.pdf")
render_cover("01", content, cover_pdf, palette=palette, fonts=fonts)

# ─── 合并：封面 + 正文 ───
from pypdf import PdfReader, PdfWriter, Transformation

A4_W, A4_H = 595.28, 841.89


def normalize_page_to_a4(page):
    box = page.mediabox
    w, h = float(box.width), float(box.height)
    if abs(w - A4_W) > 2 or abs(h - A4_H) > 2:
        sx, sy = A4_W / w, A4_H / h
        page.add_transformation(Transformation().scale(sx=sx, sy=sy))
        page.mediabox.lower_left = (0, 0)
        page.mediabox.upper_right = (A4_W, A4_H)
    return page


body_pdf = os.path.join(HERE, "body.pdf")
out_pdf = os.path.join(HERE, "Rust学习笔记.pdf")

writer = PdfWriter()
writer.add_page(normalize_page_to_a4(PdfReader(cover_pdf).pages[0]))
for page in PdfReader(body_pdf).pages:
    writer.add_page(normalize_page_to_a4(page))
writer.add_metadata({
    "/Title": "Rust 学习笔记（基于 Rustlings 题单）",
    "/Author": "Z.ai",
    "/Creator": "Z.ai",
    "/Subject": "覆盖 Rustlings 题单全部知识点的 Rust 中文学习笔记",
})
with open(out_pdf, "wb") as f:
    writer.write(f)

n = len(PdfReader(out_pdf).pages)
size_kb = os.path.getsize(out_pdf) / 1024
print("final: %s  pages=%d  %.0fKB" % (out_pdf, n, size_kb))
