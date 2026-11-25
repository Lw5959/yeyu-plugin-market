// SPDX-License-Identifier: MIT

export function collect_images(input) {
  const { images } = JSON.parse(input)
  // 文件名由内容决定。新增或调整图片的顺序时，已有链接仍指向同一份资源。
  return JSON.stringify(images.map(({ digest, extension }) =>
    `image-${digest.slice(0, 20)}.${extension}`))
}
