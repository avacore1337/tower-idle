import * as wasm_api from "@p/index"
import {
  Skill,
  WSkill,
  Item,
  WItem,
  Boost,
  WBoost,
  Crafting,
  WCrafting,
  Exploration,
  WExploration,
  Collection,
  WCollection,
  AllCollects,
  FloorTypes,
  Floor,
  WFloor,
  ActionEntry,
  Icon,
  UserSettings,
  History,
  Status,
} from "@p/index"
export type Wasm = typeof wasm_api

type Modify<T, R> = Omit<T, keyof R> & R

export interface MSkill extends WSkill, Skill {}
export interface MExploration extends WExploration, Exploration {}
export interface MCollection extends WCollection, Collection {}
export interface MCrafting extends WCrafting, Crafting {}
export interface MItem extends WItem, Item {}
export interface MBoost extends WBoost, Boost {}
/* export interface MMaterial extends WMaterial, Material {} */

/* export interface NewCrafting { */
/*   materials: MMaterial[] */
/* } */
/* export interface MCrafting extends Modify<WCrafting, NewCrafting>, Modify<Crafting, NewCrafting> {} */

let thing = FloorTypes.Starting
export interface NewFloor {
  explorations: MExploration[]
  collections: MCollection[]
  craftings: MCrafting[]
}
export interface MFloor extends Modify<WFloor, NewFloor>, Modify<Floor, NewFloor> {}

export interface World {
  skills: MSkill[]
  floors: MFloor[]
  items: MItem[]
  boosts: MBoost[]
  current_floor: FloorTypes
  icons: Icon[]
  status: Status
}

export interface State {
  wasm: Wasm
  world: World
  history: History
  user_settings: UserSettings
  action_queue: ActionEntry[]
  numberFormat: string
  messages: string[]
}
