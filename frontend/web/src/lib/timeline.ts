export type CategoryType = 'generic' | 'unknown'

export interface CategoryConfig {
  icon: string
  color: string
}

export const categoryMap: Record<CategoryType, CategoryConfig> = {
  generic: { icon: '📝', color: '#6366f1' },
  unknown: { icon: '❓', color: '#6b7280' }
}

export const getCategoryConfig = (category: string): CategoryConfig => {
  return categoryMap[category as CategoryType] || categoryMap.unknown
}
