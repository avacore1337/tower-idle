import { library } from "@fortawesome/fontawesome-svg-core"
import { IconDefinition, IconName, IconPrefix } from "@fortawesome/fontawesome-common-types"

export function add_custom_icons() {
  let faListOldStyle: IconDefinition = {
    prefix: "fac" as IconPrefix,
    iconName: "list-old-style" as IconName,
    icon: [
      1568,
      1568,
      [],
      "e001",
      "M256 1312v192q0 13-9.5 22.5t-22.5 9.5h-192q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h192q13 0 22.5 9.5t9.5 22.5zm0-384v192q0 13-9.5 22.5t-22.5 9.5h-192q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h192q13 0 22.5 9.5t9.5 22.5zm0-384v192q0 13-9.5 22.5t-22.5 9.5h-192q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h192q13 0 22.5 9.5t9.5 22.5zm1536 768v192q0 13-9.5 22.5t-22.5 9.5h-1344q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h1344q13 0 22.5 9.5t9.5 22.5zm-1536-1152v192q0 13-9.5 22.5t-22.5 9.5h-192q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h192q13 0 22.5 9.5t9.5 22.5zm1536 768v192q0 13-9.5 22.5t-22.5 9.5h-1344q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h1344q13 0 22.5 9.5t9.5 22.5zm0-384v192q0 13-9.5 22.5t-22.5 9.5h-1344q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h1344q13 0 22.5 9.5t9.5 22.5zm0-384v192q0 13-9.5 22.5t-22.5 9.5h-1344q-13 0-22.5-9.5t-9.5-22.5v-192q0-13 9.5-22.5t22.5-9.5h1344q13 0 22.5 9.5t9.5 22.5z",
    ],
  }

  library.add(faListOldStyle)

  let crystal: IconDefinition = {
    prefix: "fac" as IconPrefix,
    iconName: "crystal" as IconName,
    icon: [
      1000,
      1000,
      [],
      "e002",
      "M485 903q6 7 15 7t15-7l190-197q6-6 6-15V309q0-9-6-15L515 97q-6-7-15-7t-15 7L295 294q-6 6-6 15v382q0 9 6 15zm20-605l67 99q2 3 2 7v192q0 4-2 7l-67 99q-2 4-5 4t-5-4l-67-99q-2-3-2-7V404q0-4 2-7l67-99q2-4 5-4t5 4zm-166 14l149-169q3-3 7.5-1.5t4.5 6.5v51q0 3-1 5L385 346q-2 2-4.5 2.5T376 347l-36-25q-2-1-2.5-4t1.5-6zm-12 51q0-4 3-5.5t7 .5l37 25q3 2 3 6v223q0 3-2 5l-38 33q-3 3-6.5 1t-3.5-6V363z",
    ],
  }

  library.add(crystal)
}
