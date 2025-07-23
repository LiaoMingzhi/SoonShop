import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * 结合 clsx 和 tailwind-merge 的工具函数
 * 用于条件性地组合CSS类名，并自动合并Tailwind CSS类
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * 创建变体类名的工具函数
 * @param variants 变体配置对象
 * @param variant 当前变体
 * @param defaultVariant 默认变体
 * @returns 对应的类名
 */
export function createVariants<T extends string>(
  variants: Record<T, string>,
  variant: T | undefined,
  defaultVariant: T
): string {
  return variants[variant || defaultVariant] || variants[defaultVariant];
}

/**
 * 条件性添加类名的工具函数
 * @param condition 条件
 * @param classes 当条件为真时添加的类名
 * @returns 类名字符串或空字符串
 */
export function conditionalClass(condition: boolean, classes: string): string {
  return condition ? classes : '';
}

/**
 * 根据状态生成类名的工具函数
 * @param state 状态值
 * @param stateClasses 状态对应的类名映射
 * @param defaultClass 默认类名
 * @returns 对应的类名
 */
export function stateClass<T extends string>(
  state: T,
  stateClasses: Record<T, string>,
  defaultClass: string = ''
): string {
  return stateClasses[state] || defaultClass;
}

/**
 * 生成动画类名的工具函数
 * @param animated 是否启用动画
 * @param animationClass 动画类名
 * @returns 动画类名或空字符串
 */
export function animationClass(animated: boolean, animationClass: string): string {
  return animated ? animationClass : '';
}

/**
 * 生成尺寸类名的工具函数
 * @param size 尺寸
 * @param sizeClasses 尺寸类名映射
 * @param defaultSize 默认尺寸
 * @returns 对应的类名
 */
export function sizeClass<T extends string>(
  size: T | undefined,
  sizeClasses: Record<T, string>,
  defaultSize: T
): string {
  return sizeClasses[size || defaultSize] || sizeClasses[defaultSize];
}

/**
 * 生成响应式类名的工具函数
 * @param breakpoint 断点
 * @param classes 类名
 * @returns 带断点前缀的类名
 */
export function responsiveClass(breakpoint: 'sm' | 'md' | 'lg' | 'xl' | '2xl', classes: string): string {
  return `${breakpoint}:${classes}`;
}

/**
 * 生成悬停效果类名的工具函数
 * @param hoverEnabled 是否启用悬停效果
 * @param hoverClasses 悬停类名
 * @returns 悬停类名或空字符串
 */
export function hoverClass(hoverEnabled: boolean, hoverClasses: string): string {
  return hoverEnabled ? hoverClasses : '';
}

/**
 * 生成焦点效果类名的工具函数
 * @param focusEnabled 是否启用焦点效果
 * @param focusClasses 焦点类名
 * @returns 焦点类名或空字符串
 */
export function focusClass(focusEnabled: boolean, focusClasses: string): string {
  return focusEnabled ? focusClasses : '';
}

/**
 * 组合多个类名生成器的工具函数
 * @param generators 类名生成器函数数组
 * @returns 组合后的类名
 */
export function combineClasses(...generators: (string | undefined | false | null)[]): string {
  return cn(generators.filter(Boolean));
}

export default cn; 