# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from astarteplatform.msghub import config_pb2 as astarteplatform_dot_msghub_dot_config__pb2
from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2


class MessageHubConfigStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.SetConfig = channel.unary_unary(
                '/astarteplatform.msghub.MessageHubConfig/SetConfig',
                request_serializer=astarteplatform_dot_msghub_dot_config__pb2.ConfigMessage.SerializeToString,
                response_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
                _registered_method=True)


class MessageHubConfigServicer(object):
    """Missing associated documentation comment in .proto file."""

    def SetConfig(self, request, context):
        """Set the configuration for the Astarte message hub. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_MessageHubConfigServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'SetConfig': grpc.unary_unary_rpc_method_handler(
                    servicer.SetConfig,
                    request_deserializer=astarteplatform_dot_msghub_dot_config__pb2.ConfigMessage.FromString,
                    response_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'astarteplatform.msghub.MessageHubConfig', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))
    server.add_registered_method_handlers('astarteplatform.msghub.MessageHubConfig', rpc_method_handlers)


 # This class is part of an EXPERIMENTAL API.
class MessageHubConfig(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def SetConfig(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(
            request,
            target,
            '/astarteplatform.msghub.MessageHubConfig/SetConfig',
            astarteplatform_dot_msghub_dot_config__pb2.ConfigMessage.SerializeToString,
            google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True)
