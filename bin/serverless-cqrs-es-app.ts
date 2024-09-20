#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { OrderAggregateStack } from '../lib/order-aggregate-stack';

const app = new cdk.App();

new OrderAggregateStack(app, "order-aggregate");