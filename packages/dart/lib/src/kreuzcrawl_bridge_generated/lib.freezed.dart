// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lib.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AuthConfig {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AuthConfig()';
}


}

/// @nodoc
class $AuthConfigCopyWith<$Res>  {
$AuthConfigCopyWith(AuthConfig _, $Res Function(AuthConfig) __);
}


/// Adds pattern-matching-related methods to [AuthConfig].
extension AuthConfigPatterns on AuthConfig {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AuthConfig_Basic value)?  basic,TResult Function( AuthConfig_Bearer value)?  bearer,TResult Function( AuthConfig_Header value)?  header,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that);case AuthConfig_Bearer() when bearer != null:
return bearer(_that);case AuthConfig_Header() when header != null:
return header(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AuthConfig_Basic value)  basic,required TResult Function( AuthConfig_Bearer value)  bearer,required TResult Function( AuthConfig_Header value)  header,}){
final _that = this;
switch (_that) {
case AuthConfig_Basic():
return basic(_that);case AuthConfig_Bearer():
return bearer(_that);case AuthConfig_Header():
return header(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AuthConfig_Basic value)?  basic,TResult? Function( AuthConfig_Bearer value)?  bearer,TResult? Function( AuthConfig_Header value)?  header,}){
final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that);case AuthConfig_Bearer() when bearer != null:
return bearer(_that);case AuthConfig_Header() when header != null:
return header(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String username,  String password)?  basic,TResult Function( String token)?  bearer,TResult Function( String name,  String value)?  header,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that.username,_that.password);case AuthConfig_Bearer() when bearer != null:
return bearer(_that.token);case AuthConfig_Header() when header != null:
return header(_that.name,_that.value);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String username,  String password)  basic,required TResult Function( String token)  bearer,required TResult Function( String name,  String value)  header,}) {final _that = this;
switch (_that) {
case AuthConfig_Basic():
return basic(_that.username,_that.password);case AuthConfig_Bearer():
return bearer(_that.token);case AuthConfig_Header():
return header(_that.name,_that.value);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String username,  String password)?  basic,TResult? Function( String token)?  bearer,TResult? Function( String name,  String value)?  header,}) {final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that.username,_that.password);case AuthConfig_Bearer() when bearer != null:
return bearer(_that.token);case AuthConfig_Header() when header != null:
return header(_that.name,_that.value);case _:
  return null;

}
}

}

/// @nodoc


class AuthConfig_Basic extends AuthConfig {
  const AuthConfig_Basic({required this.username, required this.password}): super._();
  

/// Username sent in the `Authorization: Basic` header.
 final  String username;
/// Password sent in the `Authorization: Basic` header.
 final  String password;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AuthConfig_BasicCopyWith<AuthConfig_Basic> get copyWith => _$AuthConfig_BasicCopyWithImpl<AuthConfig_Basic>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig_Basic&&(identical(other.username, username) || other.username == username)&&(identical(other.password, password) || other.password == password));
}


@override
int get hashCode => Object.hash(runtimeType,username,password);

@override
String toString() {
  return 'AuthConfig.basic(username: $username, password: $password)';
}


}

/// @nodoc
abstract mixin class $AuthConfig_BasicCopyWith<$Res> implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_BasicCopyWith(AuthConfig_Basic value, $Res Function(AuthConfig_Basic) _then) = _$AuthConfig_BasicCopyWithImpl;
@useResult
$Res call({
 String username, String password
});




}
/// @nodoc
class _$AuthConfig_BasicCopyWithImpl<$Res>
    implements $AuthConfig_BasicCopyWith<$Res> {
  _$AuthConfig_BasicCopyWithImpl(this._self, this._then);

  final AuthConfig_Basic _self;
  final $Res Function(AuthConfig_Basic) _then;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? username = null,Object? password = null,}) {
  return _then(AuthConfig_Basic(
username: null == username ? _self.username : username // ignore: cast_nullable_to_non_nullable
as String,password: null == password ? _self.password : password // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AuthConfig_Bearer extends AuthConfig {
  const AuthConfig_Bearer({required this.token}): super._();
  

/// Token sent in the `Authorization: Bearer` header.
 final  String token;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AuthConfig_BearerCopyWith<AuthConfig_Bearer> get copyWith => _$AuthConfig_BearerCopyWithImpl<AuthConfig_Bearer>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig_Bearer&&(identical(other.token, token) || other.token == token));
}


@override
int get hashCode => Object.hash(runtimeType,token);

@override
String toString() {
  return 'AuthConfig.bearer(token: $token)';
}


}

/// @nodoc
abstract mixin class $AuthConfig_BearerCopyWith<$Res> implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_BearerCopyWith(AuthConfig_Bearer value, $Res Function(AuthConfig_Bearer) _then) = _$AuthConfig_BearerCopyWithImpl;
@useResult
$Res call({
 String token
});




}
/// @nodoc
class _$AuthConfig_BearerCopyWithImpl<$Res>
    implements $AuthConfig_BearerCopyWith<$Res> {
  _$AuthConfig_BearerCopyWithImpl(this._self, this._then);

  final AuthConfig_Bearer _self;
  final $Res Function(AuthConfig_Bearer) _then;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? token = null,}) {
  return _then(AuthConfig_Bearer(
token: null == token ? _self.token : token // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AuthConfig_Header extends AuthConfig {
  const AuthConfig_Header({required this.name, required this.value}): super._();
  

/// HTTP header name to set on each request.
 final  String name;
/// HTTP header value to send.
 final  String value;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AuthConfig_HeaderCopyWith<AuthConfig_Header> get copyWith => _$AuthConfig_HeaderCopyWithImpl<AuthConfig_Header>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig_Header&&(identical(other.name, name) || other.name == name)&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,name,value);

@override
String toString() {
  return 'AuthConfig.header(name: $name, value: $value)';
}


}

/// @nodoc
abstract mixin class $AuthConfig_HeaderCopyWith<$Res> implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_HeaderCopyWith(AuthConfig_Header value, $Res Function(AuthConfig_Header) _then) = _$AuthConfig_HeaderCopyWithImpl;
@useResult
$Res call({
 String name, String value
});




}
/// @nodoc
class _$AuthConfig_HeaderCopyWithImpl<$Res>
    implements $AuthConfig_HeaderCopyWith<$Res> {
  _$AuthConfig_HeaderCopyWithImpl(this._self, this._then);

  final AuthConfig_Header _self;
  final $Res Function(AuthConfig_Header) _then;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? value = null,}) {
  return _then(AuthConfig_Header(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$CrawlEvent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'CrawlEvent()';
}


}

/// @nodoc
class $CrawlEventCopyWith<$Res>  {
$CrawlEventCopyWith(CrawlEvent _, $Res Function(CrawlEvent) __);
}


/// Adds pattern-matching-related methods to [CrawlEvent].
extension CrawlEventPatterns on CrawlEvent {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CrawlEvent_Page value)?  page,TResult Function( CrawlEvent_Error value)?  error,TResult Function( CrawlEvent_Complete value)?  complete,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that);case CrawlEvent_Error() when error != null:
return error(_that);case CrawlEvent_Complete() when complete != null:
return complete(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CrawlEvent_Page value)  page,required TResult Function( CrawlEvent_Error value)  error,required TResult Function( CrawlEvent_Complete value)  complete,}){
final _that = this;
switch (_that) {
case CrawlEvent_Page():
return page(_that);case CrawlEvent_Error():
return error(_that);case CrawlEvent_Complete():
return complete(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CrawlEvent_Page value)?  page,TResult? Function( CrawlEvent_Error value)?  error,TResult? Function( CrawlEvent_Complete value)?  complete,}){
final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that);case CrawlEvent_Error() when error != null:
return error(_that);case CrawlEvent_Complete() when complete != null:
return complete(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( CrawlPageResult field0)?  page,TResult Function( String url,  String error)?  error,TResult Function( PlatformInt64 pagesCrawled)?  complete,required TResult orElse(),}) {final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that.field0);case CrawlEvent_Error() when error != null:
return error(_that.url,_that.error);case CrawlEvent_Complete() when complete != null:
return complete(_that.pagesCrawled);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( CrawlPageResult field0)  page,required TResult Function( String url,  String error)  error,required TResult Function( PlatformInt64 pagesCrawled)  complete,}) {final _that = this;
switch (_that) {
case CrawlEvent_Page():
return page(_that.field0);case CrawlEvent_Error():
return error(_that.url,_that.error);case CrawlEvent_Complete():
return complete(_that.pagesCrawled);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( CrawlPageResult field0)?  page,TResult? Function( String url,  String error)?  error,TResult? Function( PlatformInt64 pagesCrawled)?  complete,}) {final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that.field0);case CrawlEvent_Error() when error != null:
return error(_that.url,_that.error);case CrawlEvent_Complete() when complete != null:
return complete(_that.pagesCrawled);case _:
  return null;

}
}

}

/// @nodoc


class CrawlEvent_Page extends CrawlEvent {
  const CrawlEvent_Page({required this.field0}): super._();
  

 final  CrawlPageResult field0;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CrawlEvent_PageCopyWith<CrawlEvent_Page> get copyWith => _$CrawlEvent_PageCopyWithImpl<CrawlEvent_Page>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent_Page&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'CrawlEvent.page(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $CrawlEvent_PageCopyWith<$Res> implements $CrawlEventCopyWith<$Res> {
  factory $CrawlEvent_PageCopyWith(CrawlEvent_Page value, $Res Function(CrawlEvent_Page) _then) = _$CrawlEvent_PageCopyWithImpl;
@useResult
$Res call({
 CrawlPageResult field0
});




}
/// @nodoc
class _$CrawlEvent_PageCopyWithImpl<$Res>
    implements $CrawlEvent_PageCopyWith<$Res> {
  _$CrawlEvent_PageCopyWithImpl(this._self, this._then);

  final CrawlEvent_Page _self;
  final $Res Function(CrawlEvent_Page) _then;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(CrawlEvent_Page(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as CrawlPageResult,
  ));
}


}

/// @nodoc


class CrawlEvent_Error extends CrawlEvent {
  const CrawlEvent_Error({required this.url, required this.error}): super._();
  

/// The URL that failed.
 final  String url;
/// The error message.
 final  String error;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CrawlEvent_ErrorCopyWith<CrawlEvent_Error> get copyWith => _$CrawlEvent_ErrorCopyWithImpl<CrawlEvent_Error>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent_Error&&(identical(other.url, url) || other.url == url)&&(identical(other.error, error) || other.error == error));
}


@override
int get hashCode => Object.hash(runtimeType,url,error);

@override
String toString() {
  return 'CrawlEvent.error(url: $url, error: $error)';
}


}

/// @nodoc
abstract mixin class $CrawlEvent_ErrorCopyWith<$Res> implements $CrawlEventCopyWith<$Res> {
  factory $CrawlEvent_ErrorCopyWith(CrawlEvent_Error value, $Res Function(CrawlEvent_Error) _then) = _$CrawlEvent_ErrorCopyWithImpl;
@useResult
$Res call({
 String url, String error
});




}
/// @nodoc
class _$CrawlEvent_ErrorCopyWithImpl<$Res>
    implements $CrawlEvent_ErrorCopyWith<$Res> {
  _$CrawlEvent_ErrorCopyWithImpl(this._self, this._then);

  final CrawlEvent_Error _self;
  final $Res Function(CrawlEvent_Error) _then;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? url = null,Object? error = null,}) {
  return _then(CrawlEvent_Error(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,error: null == error ? _self.error : error // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class CrawlEvent_Complete extends CrawlEvent {
  const CrawlEvent_Complete({required this.pagesCrawled}): super._();
  

/// Total number of pages crawled.
 final  PlatformInt64 pagesCrawled;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CrawlEvent_CompleteCopyWith<CrawlEvent_Complete> get copyWith => _$CrawlEvent_CompleteCopyWithImpl<CrawlEvent_Complete>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent_Complete&&(identical(other.pagesCrawled, pagesCrawled) || other.pagesCrawled == pagesCrawled));
}


@override
int get hashCode => Object.hash(runtimeType,pagesCrawled);

@override
String toString() {
  return 'CrawlEvent.complete(pagesCrawled: $pagesCrawled)';
}


}

/// @nodoc
abstract mixin class $CrawlEvent_CompleteCopyWith<$Res> implements $CrawlEventCopyWith<$Res> {
  factory $CrawlEvent_CompleteCopyWith(CrawlEvent_Complete value, $Res Function(CrawlEvent_Complete) _then) = _$CrawlEvent_CompleteCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 pagesCrawled
});




}
/// @nodoc
class _$CrawlEvent_CompleteCopyWithImpl<$Res>
    implements $CrawlEvent_CompleteCopyWith<$Res> {
  _$CrawlEvent_CompleteCopyWithImpl(this._self, this._then);

  final CrawlEvent_Complete _self;
  final $Res Function(CrawlEvent_Complete) _then;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? pagesCrawled = null,}) {
  return _then(CrawlEvent_Complete(
pagesCrawled: null == pagesCrawled ? _self.pagesCrawled : pagesCrawled // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

// dart format on
