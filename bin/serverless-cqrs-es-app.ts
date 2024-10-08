#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { OrderAggregateStack } from '../lib/order-aggregate-stack';
import { ReservationAggregateStack } from '../lib/reservation-aggregate-stack';

/* UNDER CONSTRUCTION */

const app = new cdk.App();

new OrderAggregateStack(app, "order-aggregate");

// new ReservationAggregateStack(app, "reservation-aggregate");