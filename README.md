# ASON Query

_ASON Query_ is a powerful tool for processing ASON data. It can query, manipulate, and generate new ASON data.

ASON Query also serves as an interpreter for the _ASON Query Language (AQL)_.

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [它是如何工作的？](#它是如何工作的)

<!-- /code_chunk_output -->

## 它是如何工作的？

_ASON Query_ （以下简称 _aq_）从指定的文件（或者 标准输入 stdin、管道 pipe）读取 ASON 数据，然后根据查询表达式（query expression）过滤和构建新的 ASON 数据，并输出到指定的文件（或者标准输出 stdout、管道）。

