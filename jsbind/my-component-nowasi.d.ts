import { ConnectorTypes } from './interfaces/connector-types';
import { HttpClient } from './interfaces/http-client';
import { PrintClient } from './interfaces/print-client';
import { TimeClient } from './interfaces/time-client';
import { ActUtilsConnectorTypes } from './interfaces/act-utils-connector-types';
export const connectorTypes: typeof ActUtilsConnectorTypes;
export function listTables(): string;
