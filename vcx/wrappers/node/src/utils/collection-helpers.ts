import { mapKeys, snakeCase } from 'lodash'

export type ICollectionRenameItemKeysRenameFn = (value: any, key: string) => string
const collectionRenameItemKeysDefaultRenameFn: ICollectionRenameItemKeysRenameFn = (value, key) => snakeCase(key)
export const collectionRenameItemKeys = <In extends object = object, Out extends object = object>(
  collection: In[],
  renameFn: ICollectionRenameItemKeysRenameFn = collectionRenameItemKeysDefaultRenameFn
): Out[] => collection.map(
  // TODO: Remove me once up move to the next typescript
  (item) => mapKeys(item, renameFn) as object as Out
)
