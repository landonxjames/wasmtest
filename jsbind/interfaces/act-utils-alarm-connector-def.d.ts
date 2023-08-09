export namespace ActUtilsAlarmConnectorDef {
  export function parse(input: string): AlarmEvent;
}
export interface EventDescription {
  language: string,
  latestDescription: string,
}
export type EventDescList = EventDescription[];
export interface Tag {
  key: string,
  value: string,
}
export interface Entity {
  entityValue: string,
  tags: Tag[],
}
export type EntityList = Entity[];
export interface AlarmEvent {
  eventArn: string,
  service: string,
  eventTypeCode: string,
  eventTypeCategory: string,
  startTime: string,
  endTime: string,
  eventDescription: EventDescList,
  affectedEntities: EntityList,
}
