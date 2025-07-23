import React, { forwardRef } from 'react';
import { cn } from '@/utils/classnames';

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'> {
  label?: string;
  hint?: string;
  error?: string;
  startIcon?: React.ReactNode;
  endIcon?: React.ReactNode;
  fullWidth?: boolean;
  variant?: 'default' | 'filled' | 'outlined';
  inputSize?: 'sm' | 'md' | 'lg';
}

export const Input = forwardRef<HTMLInputElement, InputProps>(({
  label,
  hint,
  error,
  startIcon,
  endIcon,
  fullWidth = false,
  variant = 'default',
  inputSize = 'md',
  className,
  ...props
}, ref) => {
  const baseClasses = `
    block rounded-xl border-0 shadow-sm ring-1 ring-inset
    placeholder:text-gray-400 focus:ring-2 focus:ring-inset
    transition-all duration-200 ease-out
    ${fullWidth ? 'w-full' : ''}
    ${startIcon ? 'pl-10' : ''}
    ${endIcon ? 'pr-10' : ''}
    ${error ? 'ring-red-300 focus:ring-red-500' : 'ring-gray-300 focus:ring-primary-500'}
  `;

  const variants = {
    default: `
      bg-gray-50 text-gray-900 focus:bg-white
      ${error ? 'bg-red-50' : ''}
    `,
    filled: `
      bg-gray-100 text-gray-900 focus:bg-white
      ${error ? 'bg-red-50' : ''}
    `,
    outlined: `
      bg-white text-gray-900 ring-2
      ${error ? 'ring-red-300 focus:ring-red-500' : 'ring-gray-300 focus:ring-primary-500'}
    `
  };

  const sizes = {
    sm: 'py-2 px-3 text-sm',
    md: 'py-3 px-4 text-base',
    lg: 'py-4 px-5 text-lg'
  };

  return (
    <div className={fullWidth ? 'w-full' : ''}>
      {label && (
        <label className="block text-sm font-medium text-gray-700 mb-2">
          {label}
        </label>
      )}
      <div className="relative">
        {startIcon && (
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <span className="text-gray-400 text-sm">{startIcon}</span>
          </div>
        )}
        <input
          ref={ref}
          className={cn(
            baseClasses,
            variants[variant],
            sizes[inputSize],
            className
          )}
          {...props}
        />
        {endIcon && (
          <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
            <span className="text-gray-400 text-sm">{endIcon}</span>
          </div>
        )}
      </div>
      {error && (
        <p className="mt-2 text-sm text-red-600 flex items-center">
          <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd"/>
          </svg>
          {error}
        </p>
      )}
      {hint && !error && (
        <p className="mt-2 text-sm text-gray-500">{hint}</p>
      )}
    </div>
  );
});

// 搜索输入框组件
export interface SearchInputProps extends Omit<InputProps, 'startIcon' | 'endIcon'> {
  onSearch?: (value: string) => void;
  onClear?: () => void;
  clearable?: boolean;
  loading?: boolean;
  searchIcon?: React.ReactNode;
  clearIcon?: React.ReactNode;
}

export const SearchInput = forwardRef<HTMLInputElement, SearchInputProps>((
  {
    onSearch,
    onClear,
    clearable = false,
    loading = false,
    searchIcon,
    clearIcon,
    className,
    ...props
  },
  ref
) => {
  const [value, setValue] = React.useState(props.value || '');
  const [isFocused, setIsFocused] = React.useState(false);

  const handleSearch = (searchValue: string) => {
    onSearch?.(searchValue);
  };

  const handleClear = () => {
    setValue('');
    onClear?.();
    onSearch?.('');
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSearch(value as string);
    }
    props.onKeyDown?.(e);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setValue(newValue);
    props.onChange?.(e);
  };

  return (
    <div className="relative">
      <Input
        {...props}
        ref={ref}
        value={value}
        onChange={handleChange}
        onKeyDown={handleKeyPress}
        onFocus={(e) => {
          setIsFocused(true);
          props.onFocus?.(e);
        }}
        onBlur={(e) => {
          setIsFocused(false);
          props.onBlur?.(e);
        }}
        startIcon={
          searchIcon || (
            <svg
              className="w-4 h-4 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
          )
        }
        endIcon={
          clearable && value && (
            <button
              type="button"
              onClick={handleClear}
              className="text-gray-400 hover:text-gray-600 focus:outline-none"
            >
              {clearIcon || (
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              )}
            </button>
          )
        }
        className={cn(
          'transition-all duration-200',
          isFocused && 'ring-2 ring-primary-500',
          className
        )}
      />
      
      {/* 搜索建议下拉框可以在这里添加 */}
    </div>
  );
});

Input.displayName = 'Input';
SearchInput.displayName = 'SearchInput'; 